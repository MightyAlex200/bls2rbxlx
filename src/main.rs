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

const SCALE: f32 = 2.;

lazy_static! {
	static ref REGULAR_BRICK_RE: Regex = Regex::new(r"(\d+?)x(\d+)(F| Base)?").unwrap();
}

// CFrame but only the rotation matrix values
// struct CFrameQuat {
// 	r00: f64,
// 	r01: f64,
// 	r02: f64,
// 	r10: f64,
// 	r11: f64,
// 	r12: f64,
// 	r20: f64,
// 	r21: f64,
// 	r22: f64,
// }

// const ROTATIONS: [CFrameQuat; 4] = [
// 	CFrameQuat {
// 		r00: 1.,
// 		r01: 0.,
// 		r02: 0.,
// 		r10: 0.,
// 		r11: 1.,
// 		r12: 0.,
// 		r20: 0.,
// 		r21: 0.,
// 		r22: 1.,
// 	},
// 	CFrameQuat {
// 		r00: 0.,
// 		r01: 0.,
// 		r02: 1.,
// 		r10: 0.,
// 		r11: 1.,
// 		r12: 0.,
// 		r20: -1.,
// 		r21: 0.,
// 		r22: 0.,
// 	},
// 	CFrameQuat {
// 		r00: -1.,
// 		r01: 0.,
// 		r02: 0.,
// 		r10: 0.,
// 		r11: 1.,
// 		r12: 0.,
// 		r20: 0.,
// 		r21: 0.,
// 		r22: -1.,
// 	},
// 	CFrameQuat {
// 		r00: 0.,
// 		r01: 0.,
// 		r02: -1.,
// 		r10: 0.,
// 		r11: 1.,
// 		r12: 0.,
// 		r20: 1.,
// 		r21: 0.,
// 		r22: 0.,
// 	},
// ];

// fn cframe_with_rot(x: f64, y: f64, z: f64, rotation: u8) -> CoordinateFrame {
// 	let CFrameQuat {
// 		r00,
// 		r01,
// 		r02,
// 		r10,
// 		r11,
// 		r12,
// 		r20,
// 		r21,
// 		r22,
// 	} = ROTATIONS[rotation as usize];
// 	CoordinateFrame {
// 		x,
// 		y,
// 		z,
// 		r00,
// 		r01,
// 		r02,
// 		r10,
// 		r11,
// 		r12,
// 		r20,
// 		r21,
// 		r22,
// 	}
// }

// fn autobricksize(name: &str) -> Option<types::size> {
// 	let captures = BRICK_SIZE_REGEX.captures(name)?;
// 	let (x, z): (f64, f64) = (
// 		captures.get(1)?.as_str().parse().ok()?,
// 		captures.get(2)?.as_str().parse().ok()?,
// 	);
// 	if let Some(_) = captures.get(3) {
// 		// F
// 		Some(types::size(Vector3 {
// 			x: x * 0.5,
// 			y: 0.2,
// 			z: z * 0.5,
// 		}))
// 	} else {
// 		Some(types::size(Vector3 {
// 			x: x * 0.5,
// 			y: 0.6,
// 			z: z * 0.5,
// 		}))
// 	}
// }

// fn get_brick_size(brick: &bl_save::BrickBase) -> types::size {
// 	BRICK_SIZES
// 		.get(&brick.ui_name)
// 		.cloned()
// 		.or_else(|| autobricksize(&brick.ui_name))
// 		.unwrap_or_else(|| {
// 			println!("UNKNOWN BRICK NAME: {}", brick.ui_name);
// 			types::size(Vector3 {
// 				x: 1.,
// 				y: 1.,
// 				z: 1.,
// 			})
// 		})
// }

// fn get_brick_cframe(brick: &bl_save::BrickBase) -> CFrame {
// 	cframe_with_rot(
// 		brick.position.0 as f64,
// 		brick.position.2 as f64,
// 		-brick.position.1 as f64,
// 		brick.angle,
// 	)
// }

fn items_from_brick(brick: bl_save::BrickBase, colors: &[(f32, f32, f32, f32); 64]) -> Vec<Item> {
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
		BrickType::Wedge { cframe, size } => unimplemented!(),
		BrickType::Unknown => {
			println!("UNKNOWN BRICK: {:?}", brick);
			vec![]
		}
	}
	// Item::new(
	// 	"Part",
	// 	types::RBXUUID(uuid::Uuid::new_v4()),
	// 	types::Name(brick.ui_name.to_string()),
	// 	get_brick_cframe(&brick),
	// 	get_brick_size(&brick),
	// 	get_brick_color(&brick, colors),
	// )
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
	} else {
		BrickType::Unknown // TODO: Wedge
	}
}

fn cframe_from_pos_and_rot(pos: (f32, f32, f32), angle: u8) -> CFrame {
	let x = pos.0 * 2. * SCALE;
	let y = pos.2 * 2. * SCALE;
	let z = -pos.1 * 2. * SCALE;
	let (r00, r01, r02, r10, r11, r12, r20, r21, r22) = match angle {
		0 => (1., 0., 0., 0., 1., 0., 0., 0., 1.),
		1 => (0., 0., 1., 0., 1., 0., -1., 0., 0.),
		2 => (-1., 0., 0., 0., 1., 0., 0., 0., -1.),
		_ => (0., 0., -1., 0., 1., 0., 1., 0., 0.),
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
	Regular { cframe: CFrame, size: Vector3 },
	Wedge { cframe: CFrame, size: Vector3 },
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
