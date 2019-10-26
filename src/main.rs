#[macro_use]
extern crate lazy_static;

use bl_save;
use regex::Regex;
use std::collections::HashMap;
use std::io::Write;
use std::iter::FromIterator;
use std::{
	fs::File,
	io::{BufReader, BufWriter},
};

mod types;
mod xml;

use types::{CFrame, Color3, Item, Property, Vector3};
use xml::*;

const SCALE: f32 = 1.;

lazy_static! {
	static ref REGULAR_BRICK_RE: Regex = Regex::new(r"(\d+?)x(\d+)(F| Base)?").unwrap();
	static ref RAMP_BRICK_RE: Regex = Regex::new(r"(\d+)° Ramp (\d+)x").unwrap();
	static ref CORNER_RAMP_BRICK_RE: Regex = Regex::new(r"(\d+)° Ramp Corner").unwrap();
}

fn items_from_brick(brick: bl_save::BrickBase, colors: &[(f32, f32, f32, f32); 64]) -> Vec<Item> {
	const WEDGE_LIP_SIZE: f32 = 0.15 * SCALE;

	match get_brick_type(&brick) {
		BrickType::Regular { cframe, size } => vec![{
			let mut item = Item::default("Part".to_string());
			item.properties
				.insert("size".to_string(), Property::Vector3(size));
			item.properties
				.insert("CFrame".to_string(), Property::CFrame(cframe));
			item.properties.insert(
				"Color3uint8".to_string(),
				Property::Color3(colors[brick.color_index as usize].into()),
			);
			item
		}],
		BrickType::Ramp { cframe, size } => vec![
			{
				// Wedge part of ramp
				let mut item = Item::default("WedgePart".to_string());
				item.properties.insert(
					"size".to_string(),
					Property::Vector3(size.clone() - Vector3::new(0., WEDGE_LIP_SIZE, 0.)),
				);
				item.properties.insert(
					"CFrame".to_string(),
					Property::CFrame(
						cframe.clone()
							+ Vector3::new(0., WEDGE_LIP_SIZE / 2., 0.)
							+ forward_from_angle(brick.angle) * SCALE * 0.5,
					),
				);
				item.properties.insert(
					"Color3uint8".to_string(),
					Property::Color3(colors[brick.color_index as usize].into()),
				);
				item
			},
			{
				// Ramp lip (bottom of ramp)
				let mut item = Item::default("Part".to_string());
				item.properties.insert(
					"size".to_string(),
					Property::Vector3(Vector3::new(size.x, WEDGE_LIP_SIZE, size.z)),
				);
				item.properties.insert(
					"CFrame".to_string(),
					Property::CFrame(
						cframe.clone()
							+ Vector3::new(0., -size.y / 2. + WEDGE_LIP_SIZE / 2., 0.)
							+ forward_from_angle(brick.angle) * SCALE * 0.5,
					),
				);
				item.properties.insert(
					"Color3uint8".to_string(),
					Property::Color3(colors[brick.color_index as usize].into()),
				);
				item
			},
			{
				// Back of ramp
				let mut item = Item::default("Part".to_string());
				item.properties.insert(
					"size".to_string(),
					Property::Vector3(Vector3::new(size.x, size.y, SCALE)),
				);
				item.properties.insert(
					"CFrame".to_string(),
					Property::CFrame(cframe - (forward_from_angle(brick.angle) * (size.z / 2.))),
				);
				item.properties.insert(
					"Color3uint8".to_string(),
					Property::Color3(colors[brick.color_index as usize].into()),
				);
				item
			},
		],
		BrickType::RampCorner {
			wedge_cframe_1,
			wedge_cframe_2,
			corner_cframe,
			size,
		} => vec![
			{
				// Corner wedge of corner ramp
				let mut item = Item::default("CornerWedgePart".to_string());
				item.properties.insert(
					"size".to_string(),
					Property::Vector3(Vector3::new(
						size.x - SCALE,
						size.y - WEDGE_LIP_SIZE,
						size.z - SCALE,
					)),
				);
				item.properties.insert(
					"CFrame".to_string(),
					Property::CFrame(
						corner_cframe.clone()
							+ forward_from_angle(brick.angle) * SCALE * 0.5
							+ right_from_angle(brick.angle) * SCALE * 0.5
							+ Vector3::new(0., WEDGE_LIP_SIZE / 2., 0.),
					),
				);
				item.properties.insert(
					"Color3uint8".to_string(),
					Property::Color3(colors[brick.color_index as usize].into()),
				);
				item
			},
			{
				// Corner of corner ramp
				let mut item = Item::default("Part".to_string());
				item.properties.insert(
					"size".to_string(),
					Property::Vector3(Vector3::new(SCALE, size.y - WEDGE_LIP_SIZE, SCALE)),
				);
				item.properties.insert(
					"CFrame".to_string(),
					Property::CFrame(
						corner_cframe.clone()
							+ forward_from_angle(brick.angle) * -SCALE
							+ right_from_angle(brick.angle) * -SCALE
							+ Vector3::new(0., WEDGE_LIP_SIZE / 2., 0.),
					),
				);
				item.properties.insert(
					"Color3uint8".to_string(),
					Property::Color3(colors[brick.color_index as usize].into()),
				);
				item
			},
			{
				// First side of the corner ramp
				let mut item = Item::default("WedgePart".to_string());
				item.properties.insert(
					"size".to_string(),
					Property::Vector3(Vector3::new(SCALE, size.y - WEDGE_LIP_SIZE, size.z - SCALE)),
				);
				item.properties.insert(
					"CFrame".to_string(),
					Property::CFrame(
						wedge_cframe_1
							+ forward_from_angle(brick.angle) * -SCALE
							+ right_from_angle(brick.angle) * SCALE * 0.5
							+ Vector3::new(0., WEDGE_LIP_SIZE / 2., 0.),
					),
				);
				item.properties.insert(
					"Color3uint8".to_string(),
					Property::Color3(colors[brick.color_index as usize].into()),
				);
				item
			},
			{
				// Second side of the corner ramp
				let mut item = Item::default("WedgePart".to_string());
				item.properties.insert(
					"size".to_string(),
					Property::Vector3(Vector3::new(SCALE, size.y - WEDGE_LIP_SIZE, size.z - SCALE)),
				);
				item.properties.insert(
					"CFrame".to_string(),
					Property::CFrame(
						wedge_cframe_2
							+ right_from_angle(brick.angle) * -SCALE
							+ forward_from_angle(brick.angle) * SCALE * 0.5
							+ Vector3::new(0., WEDGE_LIP_SIZE / 2., 0.),
					),
				);
				item.properties.insert(
					"Color3uint8".to_string(),
					Property::Color3(colors[brick.color_index as usize].into()),
				);
				item
			},
			{
				// Lip of corner ramp (bottom of ramp)
				let mut item = Item::default("Part".to_string());
				item.properties.insert(
					"size".to_string(),
					Property::Vector3(Vector3::new(size.x, WEDGE_LIP_SIZE, size.z)),
				);
				item.properties.insert(
					"CFrame".to_string(),
					Property::CFrame(
						corner_cframe + Vector3::new(0., -size.y / 2. + WEDGE_LIP_SIZE / 2., 0.),
					),
				);
				item.properties.insert(
					"Color3uint8".to_string(),
					Property::Color3(colors[brick.color_index as usize].into()),
				);
				item
			},
		],
		BrickType::Unknown => {
			println!("UNKNOWN BRICK: {:?}", brick);
			vec![]
		}
	}
}

fn get_brick_type(brick: &bl_save::BrickBase) -> BrickType {
	if let Some(caps) = REGULAR_BRICK_RE.captures(&brick.ui_name) {
		let x: f32 = caps.get(1).unwrap().as_str().parse().unwrap(); // These will never panic, check the RE
		let z: f32 = caps.get(2).unwrap().as_str().parse().unwrap();
		let y = if let Some(_) = caps.get(3) { 0.4 } else { 1.2 };
		BrickType::Regular {
			size: Vector3::new(x * SCALE, y * SCALE, z * SCALE),
			cframe: cframe_from_pos_and_rot(brick.position, brick.angle),
		}
	} else if let Some(caps) = RAMP_BRICK_RE.captures(&brick.ui_name) {
		// TODO: Be generic over first capture (angle)
		assert_eq!(caps.get(1).unwrap().as_str(), "25");

		let x = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
		let z = 2;
		let y = 1.2;
		BrickType::Ramp {
			size: Vector3::new(x as f32 * SCALE, y * SCALE, z as f32 * SCALE),
			cframe: cframe_from_pos_and_rot(brick.position, brick.angle),
		}
	} else if let Some(caps) = CORNER_RAMP_BRICK_RE.captures(&brick.ui_name) {
		// TODO: Be generic over angles
		assert_eq!(caps.get(1).unwrap().as_str(), "25");

		let x = 3.;
		let z = x;
		let y = 1.2;
		BrickType::RampCorner {
			size: Vector3::new(x * SCALE, y * SCALE, z * SCALE),
			corner_cframe: cframe_from_pos_and_rot(brick.position, (brick.angle + 2) % 4),
			wedge_cframe_1: cframe_from_pos_and_rot(brick.position, (brick.angle + 1) % 4),
			wedge_cframe_2: cframe_from_pos_and_rot(brick.position, brick.angle),
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

fn cframe_from_pos_and_rot(pos: (f32, f32, f32), angle: u8) -> CFrame {
	let x = pos.0 * 2. * SCALE;
	let y = pos.2 * 2. * SCALE;
	let z = -pos.1 * 2. * SCALE;
	let (r00, r01, r02, r10, r11, r12, r20, r21, r22) = match angle {
		0 => (1., 0., 0., 0., 1., 0., 0., 0., 1.),
		1 => (0., 0., -1., 0., 1., 0., 1., 0., 0.),
		2 => (-1., 0., 0., 0., 1., 0., 0., 0., -1.),
		_ => (0., 0., 1., 0., 1., 0., -1., 0., 0.),
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

enum BrickType {
	Regular {
		cframe: CFrame,
		size: Vector3,
	},
	Ramp {
		cframe: CFrame,
		size: Vector3,
	},
	RampCorner {
		corner_cframe: CFrame,
		wedge_cframe_1: CFrame,
		wedge_cframe_2: CFrame,
		size: Vector3,
	},
	Unknown,
}

fn main() {
	let file = BufReader::new(File::open("ToConvert.bls").unwrap());
	let reader = bl_save::Reader::new(file).unwrap();
	let colors = reader.colors().clone();
	let num_bricks = reader.brick_count().unwrap();

	let mut items = Vec::<Item>::new();

	for (i, brick) in reader.into_iter().enumerate() {
		let brick = brick.unwrap();
		for item in items_from_brick(brick.base, &colors) {
			items.push(item);
		}
		println!(
			"{} bricks processed ({}%)",
			i,
			(i + 1) as f32 / num_bricks as f32 * 100.
		)
	}

	let result_file = File::create("./result.rbxlx").unwrap();
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
