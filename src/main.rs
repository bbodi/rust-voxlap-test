extern crate ascii;
extern crate sdl2;
extern crate voxlap;
extern crate rand; 

use rand::thread_rng;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use rand::Rng;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;
use ascii::AsciiStr;

use voxlap::Voxlap;
use voxlap::Orientation;
use voxlap::vec3;
use voxlap::ivec3;
use voxlap::RenderDestination;

use chart::Chart;
use plasma::PlasmaManager;
use heightmap::generate_heightmap;
use heightmap::create_grass;

mod chart;
mod plasma;
mod heightmap;
mod voxelizer;

const SCREEN_WIDHT: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

struct UserInput {
	strafe: f32,
	forward: f32,
	rot_around_z: f32,
	rot_around_right_vec: f32,
	m1_pressed: bool,
}

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem.window("rust-sdl2 demo", SCREEN_WIDHT, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

	window.set_size(1280, 900);
	window.set_position(sdl2::video::WindowPos::Centered, sdl2::video::WindowPos::Centered);

    let mut canvas = window.into_canvas().software().build().unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator.create_texture(PixelFormatEnum::ARGB8888, TextureAccess::Streaming, SCREEN_WIDHT, SCREEN_HEIGHT).unwrap();

	sdl_context.mouse().set_relative_mouse_mode(true);

    let mut voxlap = Voxlap::new().unwrap();
	voxlap::kz_addstack("data.zip");
	let vsid = voxlap.get_max_xy_dimension();

	let mut ori = voxlap.load_vxl("untitled.vxl").unwrap();
	voxlap.load_sky("BLUE").unwrap();

	let mut animated_sprite = voxlap::Sprite::new("anasplit.kfa");
	animated_sprite.set_pos(&vec3::newi(500, 200, -100));

	let rust_logo_model = load_rust_logo(&mut voxlap);

	create_shapes_into_vxl(&mut voxlap);


	let heightmap_buffer = generate_heightmap(257, 257, 6);
	voxlap.set_heightmap(heightmap_buffer.as_slice(), 257, 257, 0, 800);

	create_grass(&mut voxlap, 900, 0, 1024, 600);

	let rust_logo = voxlap::load_image("rust_logo_little.png");
	let ascii_img = voxlap::load_image("kasci9x12.png");

	let front_img = voxlap::load_image("soldier_front.png");
	let right_img = voxlap::load_image("soldier_right.png");
	let back_img = voxlap::load_image("soldier_back.png");

	voxelizer::voxelize(&mut voxlap, &front_img, &right_img, &back_img, ivec3::new(780, 470, 81));
	voxelizer::voxelize(&mut voxlap, &front_img, &right_img, &back_img, ivec3::new(678, 470, 81));

	write_thanks_message(&mut voxlap);

    let mut rust_is_awesome_buffer = voxlap::RenderDestination::new(120, 80);
    {
        let text_buffer_context = voxlap.set_frame_buffer(&mut rust_is_awesome_buffer);
        text_buffer_context.print6x8(0, 0, voxlap::Color::white(), None, "Rust is awesome!");
    }

	let mut light_mode = voxlap::LightingMode::SimpleEstimatedNormalLighting;
	voxlap.set_lighting_mode(light_mode);
	voxlap.set_fog_color(voxlap::Color::rgb(50, 50, 50));
	voxlap.generate_vxl_mipmapping(0, 0, vsid, vsid);
	voxlap.update_vxl();
	let mut max_scan_dist = 1000;
	voxlap.set_max_scan_dist(max_scan_dist);


	let mut plasma_manager = PlasmaManager::new();
	let mut frame_count = 0u32;
	let mut next_frame_tick = 0;
	let mut current_plasma_type = plasma::PlasmaType::Single(10);

	let mut chart = Chart::new()
						.x(0)
						.y(SCREEN_HEIGHT-110)
						.max_elem_count(100)
						.max_height(100)
						.column_width(3);
	let mut next_click_allowed_tick = 0;
    let mut last_hit_pos_and_color: (Option<ivec3>, Option<voxlap::Color>) = (None, None);
    let mut timer = sdl_context.timer().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
	'main : loop {
		let current_tick = timer.ticks();
		let _ = texture.with_lock(None, |c_buffer, pitch| {
			let mut render_dest = RenderDestination::from_bytes(c_buffer, SCREEN_WIDHT, SCREEN_HEIGHT, pitch as u32);
			let mut render_context = voxlap.set_frame_buffer(&mut render_dest);
			render_context.set_camera(&ori, 1f32);
			render_context.opticast();
			render_context.draw_sprite(&rust_logo_model);

			animated_sprite.animate(10);
			render_context.draw_sprite(&animated_sprite);

			chart.draw(&render_context);

			plasma_manager.update(&mut voxlap, current_tick as u32);
			plasma_manager.draw_plasmas(&render_context);
			plasma_manager.draw_falling_sprites(&render_context);


			let radius = (current_tick & 0b111100000) / 100;
			for x in 0 .. rust_is_awesome_buffer.width() {
				for z in 0 .. rust_is_awesome_buffer.height() {
					if rust_is_awesome_buffer.get(x, z) != voxlap::Color::rgb(0, 0, 0) {
						render_context.draw_sphere_with_z_buffer(&vec3::new(-200f32+((x as f32)*10f32), -500f32, 50f32+(z as f32)*10f32), 3f32 + radius as f32, voxlap::Color::rgb(0, 0, 0));
					}
				}
			}
			render_context.print6x8(10, 10, voxlap::Color::white(), None, &format!("x: {}, y: {}, z: {}", ori.pos.x, ori.pos.y, ori.pos.z)[..]);
			print_hotkey_action(&mut render_context, 10, 20, "(U/J)", &format!("raycast density: {}", voxlap.get_raycast_density())[..]);
			print_hotkey_action(&mut render_context, 10, 30, "(R/F)", &format!("max_scan_dist: {}", max_scan_dist)[..]);
			print_hotkey_action(&mut render_context, 10, 40, "(1-3)", &format!("lighting mode: {:?}", light_mode)[..]);
			print_hotkey_action(&mut render_context, 10, 50, "(5-8)", &format!("Weapon: {:?}", current_plasma_type )[..]);
			print_hotkey_action(&mut render_context, 10, 60, "(L)", "Placing light source");
			print_hotkey_action(&mut render_context, 10, 70, "(LMB)", "Fire");
			let (last_hit_pos, _) = last_hit_pos_and_color;
			if last_hit_pos.is_some() && (last_hit_pos.unwrap().to_vec3() - ori.pos).len() < 60f32 {
				let last_hit_pos = last_hit_pos.unwrap();
				render_context.print6x8(SCREEN_WIDHT/2+30, SCREEN_HEIGHT/2, voxlap::Color::white(), Some(voxlap::Color::rgb(150, 0, 0)), &format!("{}", last_hit_pos.x)[..]);
				render_context.print6x8(SCREEN_WIDHT/2+60, SCREEN_HEIGHT/2, voxlap::Color::white(), Some(voxlap::Color::rgb(0, 150, 0)), &format!("{}", last_hit_pos.y)[..]);
				render_context.print6x8(SCREEN_WIDHT/2+90, SCREEN_HEIGHT/2, voxlap::Color::white(), Some(voxlap::Color::rgb(0, 0, 150)), &format!("{}", last_hit_pos.z)[..]);
			}

			for (i, ch) in AsciiStr::from_ascii("Voxlap Binding for Rust").unwrap().as_bytes().iter().enumerate() {
				voxlap::draw_tile()
					.tile_width(9)
					.tile_height(12)
					.screen_x((SCREEN_WIDHT-250+(i as u32)*9) as u32)
					.screen_y(SCREEN_HEIGHT-20)
					.tile_per_row(1)
					.row(*ch as u32 - 32)
					.draw(&ascii_img, &render_context);
			}
			render_context.draw_image_2d(&rust_logo, SCREEN_WIDHT-40, SCREEN_HEIGHT-40, 30, 30);

			// TODO: sprhitscan does not work
			/* match voxlap.sprhitscan(&ori.pos, &ori.forward_vec, &rust_logo_model) {
				None => {},
				Some(hit) => {
					//voxlap::set_cube(&hit.pos, Some(voxlap::Color::rgb(255, 0, 0)));
					//render_context.draw_sphere_with_z_buffer(&hit.pos.to_vec3(), 10f32, voxlap::Color::rgb(255, 0, 0));
				},
			}*/

			draw_3d_axises(&mut render_context, &ori);

			let (last_hit_pos, original_color) = last_hit_pos_and_color;
			if last_hit_pos.is_some() {
				voxlap.set_cube(&last_hit_pos.unwrap(), original_color);
			}
			let was_hit = voxlap.with_hitscan(&ori.pos, &ori.forward_vec, |_, hit| {
				last_hit_pos_and_color = (Some(hit.pos), Some(hit.get_color()));
				hit.set_color(voxlap::Color::rgb(255, 0, 0));
			});
			if !was_hit {
				last_hit_pos_and_color = (None, None);
			}
		});

	    canvas.copy(&texture, None, None).unwrap();
	    canvas.present();

		let mut input = UserInput{strafe: 0f32, forward: 0f32, rot_around_z: 0f32, rot_around_right_vec: 0f32, m1_pressed: false};
		for event in event_pump.poll_iter() {
			match event {
				sdl2::event::Event::Quit{..} => break 'main,
				_ => {},
			}
		}

		let keys = event_pump.keyboard_state();
		if keys.is_scancode_pressed(Scancode::Escape) {
			break 'main;
		}
		if keys.is_scancode_pressed(Scancode::R) {
			max_scan_dist = max_scan_dist + 10;
			voxlap.set_max_scan_dist(max_scan_dist);
		} else if keys.is_scancode_pressed(Scancode::F) {
			max_scan_dist = max_scan_dist - 10;
			voxlap.set_max_scan_dist(max_scan_dist);
		}
		let mut speedmult = 1f32;
		if keys.is_scancode_pressed(Scancode::LShift) {
			speedmult = 2f32;
		}
		if keys.is_scancode_pressed(Scancode::W) {
			input.forward = 5f32 * speedmult;
		} else if keys.is_scancode_pressed(Scancode::S) {
			input.forward = -5f32 * speedmult;
		}
		if keys.is_scancode_pressed(Scancode::A) {
			input.strafe = -5f32 * speedmult;
		} else if keys.is_scancode_pressed(Scancode::D) {
			input.strafe = 5f32 * speedmult;
		}
		if keys.is_scancode_pressed(Scancode::Num1) {
			voxlap.set_lighting_mode(voxlap::LightingMode::NoSpecialLighting);
			light_mode = voxlap::LightingMode::NoSpecialLighting;
		}  else if keys.is_scancode_pressed(Scancode::Num2) {
			voxlap.set_lighting_mode(voxlap::LightingMode::SimpleEstimatedNormalLighting);
			light_mode = voxlap::LightingMode::SimpleEstimatedNormalLighting;
		} else if keys.is_scancode_pressed(Scancode::Num3) {
			voxlap.set_lighting_mode(voxlap::LightingMode::MultiplePointSourceLighting);
			light_mode = voxlap::LightingMode::MultiplePointSourceLighting;
		}else if keys.is_scancode_pressed(Scancode::Num5) {
			current_plasma_type = plasma::PlasmaType::Single(10);
		}else if keys.is_scancode_pressed(Scancode::Num6) {
			current_plasma_type = plasma::PlasmaType::Multi(10);
		}else if keys.is_scancode_pressed(Scancode::Num7) {
			current_plasma_type = plasma::PlasmaType::Rapid;
		} else if keys.is_scancode_pressed(Scancode::Num8) {
			current_plasma_type = plasma::PlasmaType::Bomb;
		}

		if keys.is_scancode_pressed(Scancode::L) {
			voxlap.set_norm_flash(&ori.pos, 256, 8192);
		} else if keys.is_scancode_pressed(Scancode::U) {
			let cur_density = voxlap.get_raycast_density();
			voxlap.set_raycast_density(cur_density + 1);
		} else if keys.is_scancode_pressed(Scancode::J) {
			let cur_density = voxlap.get_raycast_density();
			if cur_density > 1 {
				voxlap.set_raycast_density(cur_density - 1);
			}
		}
		if keys.is_scancode_pressed(Scancode::Left) {
			input.rot_around_z = -10f32 / 100f32;
		} else if keys.is_scancode_pressed(Scancode::Right) {
			input.rot_around_z = 10f32 / 100f32;
		}

		let state = event_pump.relative_mouse_state();
		let xrel = state.x();
		let yrel = state.y();
		input.m1_pressed = state.is_mouse_button_pressed(MouseButton::Left);
		input.rot_around_z = xrel as f32 / 100f32;
		input.rot_around_right_vec = (-yrel as f32) / 100f32;

		move_cam(&mut voxlap, &mut ori, &input);

		if input.m1_pressed && (next_click_allowed_tick < current_tick) {
			next_click_allowed_tick = current_tick + current_plasma_type.get_click_delay();
			plasma_manager.add_plasma(&ori.pos, &ori.forward_vec, current_tick as u32, current_plasma_type);
		}

		canvas.clear();
		frame_count += 1;

		if current_tick >= next_frame_tick {
			next_frame_tick = current_tick + 1000;
			chart.add_data(frame_count);
			frame_count = 0;
		}
	}
}

fn print_hotkey_action(render_context: &mut voxlap::RenderContext, x: u32, y: u32, hotkey: &str, descr: &str) {
	render_context.print6x8(x, y, voxlap::Color::white(), Some(voxlap::Color::black()), hotkey);
	render_context.print6x8(x+36, y, voxlap::Color::white(), None, descr);
}

fn move_cam(voxlap: &Voxlap, ori: &mut Orientation, input: &UserInput) {
	voxlap::z_rotate(&mut ori.forward_vec, input.rot_around_z);
	voxlap::z_rotate(&mut ori.down_vec, input.rot_around_z);
	voxlap::z_rotate(&mut ori.right_vec, input.rot_around_z);

	let axis = ori.right_vec.clone();
	voxlap::axis_rotate(&mut ori.forward_vec, &axis, input.rot_around_right_vec);
	voxlap::axis_rotate(&mut ori.down_vec, &axis, input.rot_around_right_vec);
	voxlap::axis_rotate(&mut ori.right_vec, &axis, input.rot_around_right_vec);

	let vec = vec3 {
		x: input.forward * ori.forward_vec.x + input.strafe * ori.right_vec.x,
		y: input.forward * ori.forward_vec.y + input.strafe * ori.right_vec.y,
		z: input.forward * ori.forward_vec.z + input.strafe * ori.right_vec.z
	};
	voxlap.clip_move(&mut ori.pos, &vec, 8.0f64);
}

pub fn write_thanks_message(voxlap: &mut Voxlap) {
	let mut thanks_to_ken_buffer = voxlap::RenderDestination::new(120, 300);
	{
		let text_buffer_context = voxlap.set_frame_buffer(&mut thanks_to_ken_buffer);
		text_buffer_context.print6x8(0, 0, voxlap::Color::white(), None, "Thanks to");
		text_buffer_context.print6x8(0, 8, voxlap::Color::white(), None, "Ken Silverman");
		text_buffer_context.print6x8(0, 16, voxlap::Color::white(), None, "for his awesome");
		text_buffer_context.print6x8(0, 24, voxlap::Color::white(), None, "Voxel engine!");
	}
	for x in 0 .. thanks_to_ken_buffer.width() {
		for y in 0 .. thanks_to_ken_buffer.height() {
			if thanks_to_ken_buffer.get(x, y) != voxlap::Color::rgb(0, 0, 0) {
				voxlap.set_cube(&ivec3::new((805) as i32, (520+x) as i32, (80+y) as i32), Some(voxlap::Color::rgb(255, 0, 0)) );
			}
		}
	}
}

fn draw_3d_axises(render_context: &mut voxlap::RenderContext, ori: &Orientation) {
	let origo = ori.pos + ori.forward_vec*2f32 + ori.right_vec*1.3f32;
	let axis_len = 0.5f32;
	let x_axis = origo + vec3::new(axis_len, 0f32, 0f32);
	let y_axis = origo + vec3::new(0f32, axis_len, 0f32);
	let z_axis = origo + vec3::new(0f32, 0f32, axis_len);
	render_context.draw_line_3d_without_z_buffer(&origo, &x_axis, voxlap::Color::rgb(255, 0, 0));
	render_context.draw_line_3d_without_z_buffer(&origo, &y_axis, voxlap::Color::rgb(0, 255, 0));
	render_context.draw_line_3d_without_z_buffer(&origo, &z_axis, voxlap::Color::rgb(0, 0, 255));
}

fn load_rust_logo(voxlap: &mut Voxlap) -> voxlap::Sprite {
	let mut rust_logo_model = voxlap::Sprite::new("rust_logo2.kv6");
	rust_logo_model.set_pos(&vec3::newi(575, 600, 40));
	rust_logo_model.set_scale(0.5f32, 0.5f32, 0.5f32);
	rust_logo_model.rotate(&vec3::new(0f32, 0f32, 1f32), 33f32);
	voxlap.set_kv6_into_vxl_memory(&rust_logo_model, voxlap::CsgOperationType::Remove);
	return rust_logo_model;
}

fn create_shapes_into_vxl(voxlap: &mut Voxlap) {
	let mut rng = thread_rng();
	voxlap.set_elliposid(&ivec3::new(200, 700, 50), &ivec3::new(400, 700, 50), 10, voxlap::CsgOperationType::Insert);
	voxlap.set_cylinder(&ivec3::new(200, 750, 50), &ivec3::new(400, 750, 50), 10, voxlap::CsgOperationType::Insert);
	voxlap.set_triangle(&ivec3::new(200, 800, 20), &ivec3::new(400, 800, 20), &ivec3::new(450, 820, 50));
	let cube_vertices = vec![
		ivec3::new(200, 800, 20), ivec3::new(230, 800, 20), ivec3::new(230, 800, 80), ivec3::new(200, 800, 80),
	];
	voxlap.set_sector(cube_vertices.as_slice(), vec![1, 2, 3, 0].as_slice(), 5f32, voxlap::CsgOperationType::Insert);
	let mut spans = vec![];
	for y in 0 .. 10u8 {
		for x in 0 .. 10u8 {
			spans.push(voxlap::vspans {
				z0: 0,
				z1: rng.gen_range(30, 50),
				x: x,
				y: y
			});
		}
	}
	voxlap.set_spans(spans.as_slice(), &ivec3::new(230, 600, 20), voxlap::CsgOperationType::Insert);
}
