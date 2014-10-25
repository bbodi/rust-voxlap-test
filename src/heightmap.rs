extern crate voxlap;

use std::rand::random;
use std::rand::task_rng;
use std::rand::Rng;
use std::default::Default;
use std::cmp::max;

use voxlap::ivec3;



pub fn generate_heightmap(width: uint, height: uint, max_diff: i32) -> Vec<u8> {
	let mut rng = task_rng();
	let a = rng.gen_range::<u8>(10, 200);
	let b = rng.gen_range::<u8>(10, 200);
	let c = rng.gen_range::<u8>(10, 200);
	let d = rng.gen_range::<u8>(10, 200);

	let mut heightmap_buffer = Buffer2D::<u8>::new(width, height);
	heightmap_buffer.set(0, 0, a);
	heightmap_buffer.set(width-1, 0, b);
	heightmap_buffer.set(0, height-1, c);
	heightmap_buffer.set(width-1, height-1, d);
	diamond_square(0, 0, width, height, &mut heightmap_buffer, max_diff);

	return heightmap_buffer.buffer;
}

struct Buffer2D<T>	{
	buffer: Vec<T>,
	width: uint,
}

impl<T: Clone + Default> Buffer2D<T> {
	fn new(width: uint, height: uint) -> Buffer2D<T>  {
		let mut buff = Vec::<T>::with_capacity((width * height) as uint);
		for _ in range(0, width * height) {
			let value: T = Default::default();
			buff.push(value);
		}
		return Buffer2D {
			buffer: buff,
			width: width,
		};
	}

	fn get(&self, x: uint, y: uint) -> T {
		let index = y * self.width + x;
		self.buffer[index].clone()
	}

	fn set(&mut self, x: uint, y: uint, value: T) {
		let index = y * self.width + x;
		self.buffer[index] = value;
	}
}

fn diamond_step(offset_x: uint, offset_y: uint, width: uint, height: uint, buff: &mut Buffer2D<u8>, max_diff: i32) {
	let a = buff.get(offset_x, 				offset_y);
	let b = buff.get(offset_x + width-1, 	offset_y);
	let c = buff.get(offset_x, 				offset_y + height-1);
	let d = buff.get(offset_x + width-1, 	offset_y + height-1);
	let e = (max(0, (a + b + c + d) as i32 / 4 + (random::<i32>()%max_diff - (max_diff/2)))) as u8;
	buff.set(offset_x + width/2, offset_y + height/2, e as u8);
}

fn square_step(offset_x: uint, offset_y: uint, width: uint, height: uint, buff: &mut Buffer2D<u8>, max_diff: i32) {
	let w_half = width / 2;
	let h_half = height / 2;

	let a = buff.get(offset_x, 				offset_y);
	let b = buff.get(offset_x + width-1, 	offset_y);
	let c = buff.get(offset_x, 				offset_y + height-1);
	let d = buff.get(offset_x + width-1, 	offset_y + height-1);
	let e = buff.get(offset_x + w_half,	offset_y + h_half);
	let f = (a + c + e) / 3;
	let g = (a + b + e) / 3;
	let h = (b + d + e) / 3;
	let i = (c + d + e) / 3;
	buff.set(offset_x, 				offset_y + h_half, 	f as u8);
	buff.set(offset_x + w_half, 	offset_y, 				g as u8);
	buff.set(offset_x + width-1, 	offset_y + h_half, 	h as u8);
	buff.set(offset_x + w_half, 	offset_y + height-1, 	i as u8);
}

fn diamond_square(offset_x: uint, offset_y: uint, width: uint, height: uint, buff: &mut Buffer2D<u8>, max_diff: i32) {
	diamond_step(offset_x, offset_y, width, height, buff, max_diff);
	square_step(offset_x, offset_y, width, height, buff, max_diff);

	let w_half = width / 2 + 1;
	let h_half = height / 2 + 1;
	if w_half <= 2 || h_half <= 2  {
		return;
	}
	diamond_square(offset_x, 			offset_y, 				w_half, h_half, buff, max_diff);
	diamond_square(offset_x + w_half-1, offset_y, 				w_half, h_half, buff, max_diff);
	diamond_square(offset_x, 			offset_y + h_half-1,	w_half, h_half, buff, max_diff);
	diamond_square(offset_x + w_half-1, offset_y + h_half-1, 	w_half, h_half, buff, max_diff);
}

pub fn create_grass(voxlap: &mut voxlap::Voxlap, x1: u32, y1: u32, x2: u32, y2: u32) {
	let mut rng = task_rng();
	for x in range(x1, x2) {
		for y in range(y1, y2) {
			let height = rng.gen_range::<i32>(0, 10);
			for z in range(0, height) {
				let (r, g, b) = if z < 7 {
					(
						rng.gen_range::<u8>(0, 100),
						rng.gen_range::<u8>(150, 255),
						rng.gen_range::<u8>(0, 100)
					)
				} else {
					(
						rng.gen_range::<u8>(0, 100),
						rng.gen_range::<u8>(200, 255),
						rng.gen_range::<u8>(0, 100)
					)
				};
				voxlap.set_cube(&ivec3::new(x as i32, y as i32, 127 - z), Some(voxlap::Color::rgb(r, g, b)));
			}
		}
	}
}
