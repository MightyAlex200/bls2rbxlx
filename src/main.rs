#[macro_use]
extern crate lazy_static;

mod types;
mod xml;

use types::{CFrame, Color3, Item, Property, Vector3};

use bl_save;
use regex::Regex;
use structopt::StructOpt;
use xml::*;

use std::{
	collections::HashSet,
	fs::File,
	io::Write,
	io::{BufReader, BufWriter},
	path::PathBuf,
};

const BRICK_HEIGHT: f32 = 1.2;

lazy_static! {
	// TODO: Cones, ramp crests, lights
	static ref TALL_BRICK_RE: Regex = Regex::new(r"^(\d+)x(\d+)x(\d+)( Print)?$").unwrap();
	static ref REGULAR_BRICK_RE: Regex = Regex::new(r"^(\d+?)x(\d+)(F| Base)?( Round)?(?: Print)?$").unwrap();
	static ref RAMP_BRICK_RE: Regex = Regex::new(r"^(-)?(\d+)° Ramp (\d+)x(?: Print)?$").unwrap();
	static ref CORNER_RAMP_BRICK_RE: Regex = Regex::new(r"^(-)?(\d+)° Ramp Corner$").unwrap();
}

fn items_from_brick(
	brick: &bl_save::BrickBase,
	colors: &[(f32, f32, f32, f32); 64],
	scale: f32,
) -> Result<Vec<Item>, ()> {
	let wedge_lip_size: f32 = 0.15 * scale;

	let insert_color = |item: &mut Item| {
		let color: Color3 = colors[brick.color_index as usize].into();
		item.properties
			.insert("Color3uint8".to_string(), Property::Color3(color.clone()));
		item.properties.insert(
			"Transparency".to_string(),
			Property::Float(if !brick.rendering {
				1.
			} else {
				1. - (color.a as f32 / 255.)
			}),
		);
		item.properties
			.insert("CanCollide".to_string(), Property::Bool(brick.collision));
	};

	match get_brick_type(&brick, scale) {
		BrickType::Regular { cframe, size, mesh } => Ok(vec![{
			let mut item = Item::default("Part".to_string());
			item.properties
				.insert("size".to_string(), Property::Vector3(size));
			item.properties
				.insert("CFrame".to_string(), Property::CFrame(cframe));
			insert_color(&mut item);
			if let RegularBrickMesh::Round = mesh {
				item.children
					.push(Item::default("CylinderMesh".to_string()))
			}
			item
		}]),
		BrickType::Ramp {
			cframe,
			size,
			inverted,
		} => Ok(vec![
			{
				// Wedge part of ramp
				let mut item = Item::default("WedgePart".to_string());
				item.properties.insert(
					"size".to_string(),
					Property::Vector3(size.clone() - Vector3::new(0., wedge_lip_size, 0.)),
				);
				item.properties.insert(
					"CFrame".to_string(),
					Property::CFrame(
						cframe.clone()
							+ Vector3::new(
								0.,
								wedge_lip_size / if inverted { -2. } else { 2. },
								0.,
							) + forward_from_angle(brick.angle) * scale * 0.5,
					),
				);
				insert_color(&mut item);

				item
			},
			{
				// Ramp lip (bottom of ramp)
				let mut item = Item::default("Part".to_string());
				item.properties.insert(
					"size".to_string(),
					Property::Vector3(Vector3::new(size.x, wedge_lip_size, size.z)),
				);
				item.properties.insert(
					"CFrame".to_string(),
					Property::CFrame(
						cframe.clone()
							+ Vector3::new(
								0.,
								-size.y / if inverted { -2. } else { 2. }
									+ wedge_lip_size / if inverted { -2. } else { 2. },
								0.,
							) + forward_from_angle(brick.angle) * scale * 0.5,
					),
				);
				insert_color(&mut item);

				item
			},
			{
				// Back of ramp
				let mut item = Item::default("Part".to_string());
				item.properties.insert(
					"size".to_string(),
					Property::Vector3(Vector3::new(size.x, size.y, scale)),
				);
				item.properties.insert(
					"CFrame".to_string(),
					Property::CFrame(cframe - (forward_from_angle(brick.angle) * (size.z / 2.))),
				);
				insert_color(&mut item);

				item
			},
		]),
		BrickType::RampCorner {
			wedge_cframe_1,
			wedge_cframe_2,
			corner_cframe,
			size,
			inverted,
		} => {
			let wedge_offset =
				Vector3::new(0., wedge_lip_size / 2., 0.) * if inverted { -1. } else { 1. };
			Ok(vec![
				{
					// Corner wedge of corner ramp
					let mut item = Item::default("CornerWedgePart".to_string());
					item.properties.insert(
						"size".to_string(),
						Property::Vector3(Vector3::new(
							size.x - scale,
							size.y - wedge_lip_size,
							size.z - scale,
						)),
					);
					item.properties.insert(
						"CFrame".to_string(),
						Property::CFrame(
							corner_cframe.clone()
								+ forward_from_angle(brick.angle) * scale * 0.5
								+ right_from_angle(brick.angle) * scale * 0.5
								+ wedge_offset.clone(),
						),
					);
					insert_color(&mut item);

					item
				},
				{
					// Corner of corner ramp
					let mut item = Item::default("Part".to_string());
					item.properties.insert(
						"size".to_string(),
						Property::Vector3(Vector3::new(scale, size.y - wedge_lip_size, scale)),
					);
					item.properties.insert(
						"CFrame".to_string(),
						Property::CFrame(
							corner_cframe.clone()
								+ forward_from_angle(brick.angle) * (-size.x / 2.)
								+ forward_from_angle(brick.angle) * scale * 0.5
								+ right_from_angle(brick.angle) * (-size.z / 2.)
								+ right_from_angle(brick.angle) * scale * 0.5
								+ wedge_offset.clone(),
						),
					);
					insert_color(&mut item);

					item
				},
				{
					// First side of the corner ramp
					let mut item = Item::default("WedgePart".to_string());
					item.properties.insert(
						"size".to_string(),
						Property::Vector3(Vector3::new(
							scale,
							size.y - wedge_lip_size,
							size.z - scale,
						)),
					);
					item.properties.insert(
						"CFrame".to_string(),
						Property::CFrame(
							wedge_cframe_1
								+ forward_from_angle(brick.angle) * (-size.x / 2.)
								+ forward_from_angle(brick.angle) * scale * 0.5
								+ right_from_angle(brick.angle) * scale * 0.5
								+ wedge_offset.clone(),
						),
					);
					insert_color(&mut item);
					item
				},
				{
					// Second side of the corner ramp
					let mut item = Item::default("WedgePart".to_string());
					item.properties.insert(
						"size".to_string(),
						Property::Vector3(Vector3::new(
							scale,
							size.y - wedge_lip_size,
							size.z - scale,
						)),
					);
					item.properties.insert(
						"CFrame".to_string(),
						Property::CFrame(
							wedge_cframe_2
								+ forward_from_angle(brick.angle) * scale * 0.5
								+ right_from_angle(brick.angle) * (-size.z / 2.)
								+ right_from_angle(brick.angle) * scale * 0.5
								+ wedge_offset,
						),
					);
					insert_color(&mut item);
					item
				},
				{
					// Lip of corner ramp (bottom of ramp)
					let mut item = Item::default("Part".to_string());
					item.properties.insert(
						"size".to_string(),
						Property::Vector3(Vector3::new(size.x, wedge_lip_size, size.z)),
					);
					item.properties.insert(
						"CFrame".to_string(),
						Property::CFrame(
							corner_cframe
								+ Vector3::new(
									0.,
									size.y / if inverted { 2. } else { -2. }
										+ wedge_lip_size / if inverted { -2. } else { 2. },
									0.,
								),
						),
					);
					insert_color(&mut item);
					item
				},
			])
		}
		BrickType::Unknown => Err(()),
	}
}

fn get_brick_type(brick: &bl_save::BrickBase, scale: f32) -> BrickType {
	if let Some(caps) = TALL_BRICK_RE.captures(&brick.ui_name) {
		let x: f32 = caps.get(1).unwrap().as_str().parse().unwrap();
		let z: f32 = caps.get(2).unwrap().as_str().parse().unwrap();
		let y = caps.get(3).unwrap().as_str().parse::<f32>().unwrap() * BRICK_HEIGHT;
		BrickType::Regular {
			size: Vector3::new(x, y, z) * scale,
			cframe: cframe_from_pos_and_rot(
				brick.position,
				(brick.angle + if caps.get(4).is_some() { 1 } else { 0 }) % 4,
				false,
				scale,
			),
			mesh: RegularBrickMesh::Block,
		}
	} else if let Some(caps) = REGULAR_BRICK_RE.captures(&brick.ui_name) {
		let x: f32 = caps.get(1).unwrap().as_str().parse().unwrap(); // These will never panic, check the RE
		let z: f32 = caps.get(2).unwrap().as_str().parse().unwrap();
		let y = if let Some(_) = caps.get(3) {
			0.4
		} else {
			BRICK_HEIGHT
		};
		BrickType::Regular {
			size: Vector3::new(x, y, z) * scale,
			cframe: cframe_from_pos_and_rot(brick.position, brick.angle, false, scale),
			mesh: if caps.get(4).is_some() {
				RegularBrickMesh::Round
			} else {
				RegularBrickMesh::Block
			},
		}
	} else if let Some(caps) = RAMP_BRICK_RE.captures(&brick.ui_name) {
		let angle = parse_ramp_angle(caps.get(2).unwrap().as_str()).expect("Unknown ramp angle");

		let x = caps.get(3).unwrap().as_str().parse::<u8>().unwrap();
		let z = match angle {
			RampAngle::Angle25 => 2,
			_ => 1,
		};
		let y = BRICK_HEIGHT
			* match angle {
				RampAngle::Angle25 | RampAngle::Angle45 => 1.,
				RampAngle::Angle72 => 3.,
				RampAngle::Angle80 => 5.,
			};
		let inverted = caps.get(1).is_some();
		BrickType::Ramp {
			size: Vector3::new(x as f32, y, z as f32) * scale,
			cframe: cframe_from_pos_and_rot(brick.position, brick.angle, inverted, scale),
			inverted,
		}
	} else if let Some(caps) = CORNER_RAMP_BRICK_RE.captures(&brick.ui_name) {
		let angle =
			parse_ramp_angle(caps.get(2).unwrap().as_str()).expect("Unknown corner ramp angle");

		let x = match angle {
			RampAngle::Angle25 => 3.,
			_ => 2.,
		};
		let z = x;
		let y = BRICK_HEIGHT
			* match angle {
				RampAngle::Angle25 | RampAngle::Angle45 => 1.,
				RampAngle::Angle72 => 3.,
				RampAngle::Angle80 => 5.,
			};
		let inverted = caps.get(1).is_some();
		BrickType::RampCorner {
			size: Vector3::new(x, y, z) * scale,
			corner_cframe: cframe_from_pos_and_rot(
				brick.position,
				(brick.angle + if inverted { 3 } else { 2 }) % 4,
				inverted,
				scale,
			),
			wedge_cframe_1: cframe_from_pos_and_rot(
				brick.position,
				(brick.angle + 1) % 4,
				inverted,
				scale,
			),
			wedge_cframe_2: cframe_from_pos_and_rot(brick.position, brick.angle, inverted, scale),
			inverted,
		}
	} else {
		BrickType::Unknown
	}
}

fn forward_from_angle(angle: u8) -> Vector3 {
	match angle {
		0 => Vector3::new(0., 0., -1.),
		1 => Vector3::new(1., 0., 0.),
		2 => Vector3::new(0., 0., 1.),
		_ => Vector3::new(-1., 0., 0.),
	}
}

fn right_from_angle(angle: u8) -> Vector3 {
	match angle {
		0 => Vector3::new(1., 0., 0.),
		1 => Vector3::new(0., 0., 1.),
		2 => Vector3::new(-1., 0., 0.),
		_ => Vector3::new(0., 0., -1.),
	}
}

fn cframe_from_pos_and_rot(pos: (f32, f32, f32), angle: u8, inverted: bool, scale: f32) -> CFrame {
	let x = pos.0 * 2. * scale;
	let y = pos.2 * 2. * scale;
	let z = -pos.1 * 2. * scale;
	let (r00, r01, r02, r10, r11, r12, r20, r21, r22) = if inverted {
		match angle {
			0 => (-1., 0., -0., 0., -1., -0., -0., -0., 1.),
			1 => (0., 0., -1., 0., -1., 0., -1., 0., 0.),
			2 => (1., 0., 0., 0., -1., 0., 0., 0., -1.),
			_ => (0., 0., 1., 0., -1., 0., 1., 0., 0.),
		}
	} else {
		match angle {
			0 => (1., 0., 0., 0., 1., 0., 0., 0., 1.),
			1 => (0., 0., -1., 0., 1., 0., 1., 0., 0.),
			2 => (-1., 0., 0., 0., 1., 0., 0., 0., -1.),
			_ => (0., 0., 1., 0., 1., 0., -1., 0., 0.),
		}
	};
	CFrame {
		x,
		y,
		z,
		r00,
		r01,
		r02,
		r10,
		r11,
		r12,
		r20,
		r21,
		r22,
	}
}

enum RampAngle {
	Angle25,
	Angle45,
	Angle72,
	Angle80,
}

fn parse_ramp_angle(s: &str) -> Option<RampAngle> {
	match s {
		"25" => Some(RampAngle::Angle25),
		"45" => Some(RampAngle::Angle45),
		"72" => Some(RampAngle::Angle72),
		"80" => Some(RampAngle::Angle80),
		_ => None,
	}
}

enum RegularBrickMesh {
	Block,
	Round,
}

enum BrickType {
	Regular {
		cframe: CFrame,
		size: Vector3,
		mesh: RegularBrickMesh,
	},
	Ramp {
		cframe: CFrame,
		size: Vector3,
		inverted: bool,
	},
	RampCorner {
		corner_cframe: CFrame,
		wedge_cframe_1: CFrame,
		wedge_cframe_2: CFrame,
		size: Vector3,
		inverted: bool,
	},
	Unknown,
}

#[derive(StructOpt)]
/// Convert .bls files (Blockland save) to .rbxlx (Roblox save)
struct Args {
	#[structopt(parse(from_os_str))]
	/// File to convert
	input: PathBuf,
	#[structopt(parse(from_os_str), default_value = "result.rbxlx")]
	/// File that will be written to
	output: PathBuf,
	#[structopt(short, long, default_value = "1")]
	scale: f32,
	#[structopt(short, long)]
	/// Show no output on the command line
	quiet: bool,
}

fn main() {
	let args = Args::from_args();
	let file = BufReader::new(File::open(&args.input).unwrap());
	let reader = bl_save::Reader::new(file).unwrap();
	let colors = reader.colors().clone();
	let num_bricks = reader.brick_count().unwrap();

	let mut items = Vec::<Item>::new();
	let mut unknown_bricks = HashSet::<String>::new();

	for (i, brick) in reader.into_iter().enumerate() {
		let brick = brick.unwrap();
		match items_from_brick(&brick.base, &colors, args.scale) {
			Ok(new_items) => {
				for item in new_items {
					items.push(item);
				}
			}
			Err(()) => {
				unknown_bricks.insert(brick.base.ui_name);
			}
		}
		if !args.quiet {
			println!(
				"{} bricks processed ({}%)",
				i + 1,
				(i + 1) as f32 / num_bricks as f32 * 100.
			);
		}
	}

	if unknown_bricks.len() > 0 && !args.quiet {
		eprintln!(
			"!! {} brick types in this file could not be converted !!",
			unknown_bricks.len()
		);
		for unknown_brick in unknown_bricks {
			eprintln!("Unknown brick type: {}", unknown_brick);
		}
	}

	let result_file = File::create(&args.output).unwrap();
	let mut result_buf = BufWriter::new(result_file);
	write!(&mut result_buf, "{}", START_XML).unwrap();
	write!(
		&mut result_buf,
		"{}",
		items
			.into_iter()
			.map(|i| i.to_string())
			.collect::<Vec::<_>>()
			.join("\n")
	)
	.unwrap();
	write!(&mut result_buf, "{}", END_XML).unwrap();
}
