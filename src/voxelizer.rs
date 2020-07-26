extern crate voxlap;

use voxlap::ivec3;
use voxlap::Image;

// TODO: create and return a Voxel Sprite
pub fn voxelize(voxlap: &mut voxlap::Voxlap, front_img: &Image, right_img: &Image, back_img: &Image, pos: ivec3) {
	for x in 0 .. front_img.width {
		for y in 0 .. front_img.height {
			for z in 0 .. right_img.width {

				let front_color = front_img.get_pixel(x, y);
				let left_color = right_img.get_pixel(z, y);
				let back_color = back_img.get_pixel(x, y);

				if voxlap::Color::rgb(152, 0, 136) == front_color {
					continue;
				}
				if voxlap::Color::rgb(152, 0, 136) == left_color {
					continue;
				}
				if voxlap::Color::rgb(152, 0, 136) == back_color {
					continue;
				}

				let mut drawing_color = None;
				let pos = ivec3::new(pos.x+x as i32, pos.y+z as i32, pos.z+y as i32);

				if front_color == left_color {
					drawing_color = Some(front_color);
				} else if voxlap.all_voxel_empty(&pos, &(pos - ivec3::new(0, z as i32, 0))) {
					drawing_color = Some(front_color);
				} else if voxlap.all_voxel_empty(&pos, &(pos - ivec3::new(x as i32, 0, 0))) {
					drawing_color = Some(left_color);
				} else if voxlap.all_voxel_empty(&pos, &(pos + ivec3::new(0, (right_img.width-z) as i32, 0))) {
					drawing_color = Some(back_color);
				}
				let transparent_color = drawing_color.is_some() && drawing_color.unwrap() == voxlap::Color::rgb(32, 156, 0);
				if transparent_color {
					continue;
				}

				voxlap.set_cube(&pos, drawing_color);
			}
		}
	}
}
