extern crate voxlap;

use rand::random;
use rand::thread_rng;
use rand::Rng;
use std::default::Default;
use std::cmp::max;

use voxlap::ivec3;



pub fn generate_heightmap(width: usize, height: usize, max_diff: i32) -> Vec<u8> {
	let mut rng = thread_rng();
	let a:u8 = rng.gen_range(10, 200);
	let b:u8 = rng.gen_range(10, 200);
	let c:u8 = rng.gen_range(10, 200);
	let d:u8 = rng.gen_range(10, 200);

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
	width: usize,
}

impl<T: Clone + Default> Buffer2D<T> {
	fn new(width: usize, height: usize) -> Buffer2D<T>  {
		let mut buff = Vec::<T>::with_capacity((width * height) as usize);
		for _ in 0 .. width * height {
			let value: T = Default::default();
			buff.push(value);
		}
		return Buffer2D {
			buffer: buff,
			width: width,
		};
	}

	fn get(&self, x: usize, y: usize) -> T {
		let index = y * self.width + x;
		self.buffer[index].clone()
	}

	fn set(&mut self, x: usize, y: usize, value: T) {
		let index = y * self.width + x;
		self.buffer[index] = value;
	}
}

fn diamond_step(offset_x: usize, offset_y: usize, width: usize, height: usize, buff: &mut Buffer2D<u8>, max_diff: i32) {
	let mut rng = rand::thread_rng();
	let a = buff.get(offset_x, 				offset_y);
	let b = buff.get(offset_x + width-1, 	offset_y);
	let c = buff.get(offset_x, 				offset_y + height-1);
	let d = buff.get(offset_x + width-1, 	offset_y + height-1);
	let e = (max(0, (a as i32 + b as i32 + c as i32 + d as i32) / 4 + rng.gen_range(-max_diff/2, max_diff/2) )) as u8;
	buff.set(offset_x + width/2, offset_y + height/2, e as u8);
}

fn square_step(offset_x: usize, offset_y: usize, width: usize, height: usize, buff: &mut Buffer2D<u8>, max_diff: i32) {
	let w_half = width / 2;
	let h_half = height / 2;

	let a = buff.get(offset_x, 				offset_y);
	let b = buff.get(offset_x + width-1, 	offset_y);
	let c = buff.get(offset_x, 				offset_y + height-1);
	let d = buff.get(offset_x + width-1, 	offset_y + height-1);
	let e = buff.get(offset_x + w_half,	offset_y + h_half);
	let f = (a as i32 + c as i32 + e as i32) / 3;
	let g = (a as i32 + b as i32 + e as i32) / 3;
	let h = (b as i32 + d as i32 + e as i32) / 3;
	let i = (c as i32 + d as i32 + e as i32) / 3;
	buff.set(offset_x, 				offset_y + h_half, 	f as u8);
	buff.set(offset_x + w_half, 	offset_y, 			g as u8);
	buff.set(offset_x + width-1, 	offset_y + h_half, 	h as u8);
	buff.set(offset_x + w_half, 	offset_y + height-1,i as u8);
}

fn diamond_square(offset_x: usize, offset_y: usize, width: usize, height: usize, buff: &mut Buffer2D<u8>, max_diff: i32) {
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
	let mut rng = thread_rng();
	for x in x1 .. x2 {
		for y in y1 .. y2 {
			let height : i32 = rng.gen_range(0, 10);
			for z in 0 .. height {
				let (r, g, b) : (u8, u8, u8) = if z < 7 {
					(
						rng.gen_range(0, 100),
						rng.gen_range(150, 255),
						rng.gen_range(0, 100)
					)
				} else {
					(
						rng.gen_range(0, 100),
						rng.gen_range(200, 255),
						rng.gen_range(0, 100)
					)
				};
				voxlap.set_cube(&ivec3::new(x as i32, y as i32, 127 - z), Some(voxlap::Color::rgb(r, g, b)));
			}
		}
	}
}
