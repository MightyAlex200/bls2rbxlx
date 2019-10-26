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

use types::{Color3uint8, CoordinateFrame, Item, Vector3};
use xml::*;

lazy_static! {
	static ref BRICK_SIZES: HashMap<String, types::size> =
		FromIterator::from_iter(vec![].into_iter());
	static ref BRICK_SIZE_REGEX: Regex = Regex::new(r"(\d+?)x(\d+)(F| Base)?").unwrap();
}

// CFrame but only the rotation matrix values
struct CFrameQuat {
	r00: f64,
	r01: f64,
	r02: f64,
	r10: f64,
	r11: f64,
	r12: f64,
	r20: f64,
	r21: f64,
	r22: f64,
}

const ROTATIONS: [CFrameQuat; 4] = [
	CFrameQuat {
		r00: 1.,
		r01: 0.,
		r02: 0.,
		r10: 0.,
		r11: 1.,
		r12: 0.,
		r20: 0.,
		r21: 0.,
		r22: 1.,
	},
	CFrameQuat {
		r00: 0.,
		r01: 0.,
		r02: 1.,
		r10: 0.,
		r11: 1.,
		r12: 0.,
		r20: -1.,
		r21: 0.,
		r22: 0.,
	},
	CFrameQuat {
		r00: -1.,
		r01: 0.,
		r02: 0.,
		r10: 0.,
		r11: 1.,
		r12: 0.,
		r20: 0.,
		r21: 0.,
		r22: -1.,
	},
	CFrameQuat {
		r00: 0.,
		r01: 0.,
		r02: -1.,
		r10: 0.,
		r11: 1.,
		r12: 0.,
		r20: 1.,
		r21: 0.,
		r22: 0.,
	},
];

fn cframe_with_rot(x: f64, y: f64, z: f64, rotation: u8) -> CoordinateFrame {
	let CFrameQuat {
		r00,
		r01,
		r02,
		r10,
		r11,
		r12,
		r20,
		r21,
		r22,
	} = ROTATIONS[rotation as usize];
	CoordinateFrame {
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

fn autobricksize(name: &str) -> Option<types::size> {
	let captures = BRICK_SIZE_REGEX.captures(name)?;
	let (x, z): (f64, f64) = (
		captures.get(1)?.as_str().parse().ok()?,
		captures.get(2)?.as_str().parse().ok()?,
	);
	if let Some(_) = captures.get(3) {
		// F
		Some(types::size(Vector3 {
			x: x * 0.5,
			y: 0.2,
			z: z * 0.5,
		}))
	} else {
		Some(types::size(Vector3 {
			x: x * 0.5,
			y: 0.6,
			z: z * 0.5,
		}))
	}
}

fn get_brick_size(brick: &bl_save::BrickBase) -> types::size {
	BRICK_SIZES
		.get(&brick.ui_name)
		.cloned()
		.or_else(|| autobricksize(&brick.ui_name))
		.unwrap_or_else(|| {
			println!("UNKNOWN BRICK NAME: {}", brick.ui_name);
			types::size(Vector3 {
				x: 1.,
				y: 1.,
				z: 1.,
			})
		})
}

fn get_brick_cframe(brick: &bl_save::BrickBase) -> CoordinateFrame {
	cframe_with_rot(
		brick.position.0 as f64,
		brick.position.2 as f64,
		brick.position.1 as f64,
		brick.angle,
	)
}

fn get_brick_color(brick: &bl_save::BrickBase, colors: &[(f32, f32, f32, f32); 64]) -> Color3uint8 {
	let color = colors[brick.color_index as usize];
	Color3uint8 {
		r: (color.0 * 255.) as u8,
		g: (color.1 * 255.) as u8,
		b: (color.2 * 255.) as u8,
		a: (color.3 * 255.) as u8,
	}
}

fn item_from_brick(brick: bl_save::BrickBase, colors: &[(f32, f32, f32, f32); 64]) -> Item {
	Item::new(
		"Part",
		types::RBXUUID(uuid::Uuid::new_v4()),
		types::Name(brick.ui_name.to_string()),
		get_brick_cframe(&brick),
		get_brick_size(&brick),
		get_brick_color(&brick, colors),
	)
}

fn main() {
	let file = BufReader::new(File::open("ToConvert.bls").unwrap());
	let reader = bl_save::Reader::new(file).unwrap();
	let colors = reader.colors().clone();
	let num_bricks = reader.brick_count().unwrap();

	let mut items = Vec::<Item>::new();

	for (i, brick) in reader.into_iter().enumerate() {
		let brick = brick.unwrap();
		items.push(item_from_brick(brick.base, &colors));
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
