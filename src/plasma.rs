extern crate voxlap;

use std::iter::range_step_inclusive;
use std::rand::random;

use voxlap::Voxlap;
use voxlap::vec3;

#[deriving(Show)]
pub enum PlasmaType {
	Single(u32),
	Multi(u32),
	Rapid,
	Bomb,
}

impl PlasmaType {
	pub fn get_click_delay(&self) -> uint {
		match *self {
			Single(_) => 1000,
			Multi(_) => 1000,
			Rapid => 200,
			Bomb => 20000,
		}
	}
}

struct Plasma {
	pos: vec3,
	dir: vec3,
	free: bool,
	born_tick: u32,
	typ: PlasmaType,
}

impl Plasma {
	fn new(pos: &vec3, dir: &vec3, now_tick: u32, typ: PlasmaType) -> Plasma {
		Plasma {
			pos: *pos,
			dir: *dir,
			free: false,
			born_tick: now_tick,
			typ: typ,
		}
	}

	fn get_size(&self) -> u32 {
		match self.typ {
			Single(level) => level,
			Multi(level) => level,
			Rapid => 4,
			Bomb => 100,
		}
	}

	fn get_speed(&self) -> f32 {
		match self.typ {
			Multi(_) => 3f32,
			Single(_) => 4f32,
			Rapid => 10f32,
			Bomb => 1f32,
		}
	}
}

struct FallingSprite {
	spr: voxlap::Sprite,
	dir: vec3
}

pub struct PlasmaManager {
	plasmas: Vec<Plasma>,
	last_nonfree_plasma_index: uint,
	falling_sprites: Vec<FallingSprite>,
	free_plasmas: uint,
	all_plasmas: uint,
}

impl PlasmaManager {

	pub fn new() -> PlasmaManager {
		PlasmaManager {
			plasmas: vec![],
			falling_sprites: vec![],
			last_nonfree_plasma_index: 0,
			free_plasmas: 0,
			all_plasmas: 0,
		}
	}

	pub fn add_plasma(&mut self, pos: &vec3, dir: &vec3, now_tick: u32, typ: PlasmaType) {
		for plasma in self.plasmas.iter_mut() {
			if plasma.free {
				plasma.pos = *pos;
				plasma.dir = *dir;
				plasma.free = false;
				plasma.born_tick = now_tick;
				plasma.typ = typ;
				self.free_plasmas = self.free_plasmas - 1;
				return;
			}
		}
		let p = Plasma::new(pos, dir, now_tick, typ);
		self.plasmas.push(p);
		self.last_nonfree_plasma_index = self.plasmas.len();
		self.all_plasmas = self.plasmas.len();
	}

	pub fn update(&mut self, voxlap: &mut Voxlap, tick: u32) {
		let new_plasmas = self.move_plasmas(voxlap, tick);
		self.handle_new_plasmas(&new_plasmas);
		self.update_falling_sprites();
	}

	fn move_plasmas(&mut self, voxlap: &mut Voxlap, tick: u32) -> Vec<Plasma> {
		let mut new_plasmas = vec![];
		for plasma in self.plasmas.iter_mut() {
			if plasma.free {
				continue;
			}
			let old_pos = plasma.pos;
			plasma.pos = plasma.pos + plasma.dir* plasma.get_speed();
			if plasma.pos.x < 0f32 || plasma.pos.x >= 1024f32 || plasma.pos.y < 0f32 || plasma.pos.y >= 1024f32 {
				self.free_plasmas = self.free_plasmas + 1;
				plasma.free = true;
				continue;
			}
			let mut destruct_plasma = false;
			let mut create_new_plasma = false;
			let mut melting_pos = None;
			if let Multi(level) = plasma.typ {
				plasma.dir = plasma.dir + vec3::new(0f32, 0f32, 0.01f32);
				if level > 1 && plasma.born_tick + 4000 < tick {
					create_new_plasma = true;
					destruct_plasma = true;
				}
			}
			if !destruct_plasma {
				if let voxlap::CannotSee(hit_pos) = voxlap.can_see(&plasma.pos, &old_pos) {
					match plasma.typ {
						Single(level) => {
							if level <= 1 {
								destruct_plasma = true;
							} else {
								plasma.typ = Single(level-2)
							}
						}
						Rapid => {destruct_plasma = true;},
						Multi(level) => {
							if level > 1 {
								create_new_plasma = true;
								destruct_plasma = true;
							}
						},
						Bomb => {destruct_plasma = true;},
					}
					melting_pos = Some(hit_pos);
				}
			}

			if let Some(hit_pos) = melting_pos {
				let size = plasma.get_size();
				let (spr, _) = voxlap.melt_sphere(&hit_pos, size);
				let mut random_dir = random::<vec3>();
				for _ in range(0, 5i32) {
					if let voxlap::CanSee = voxlap.can_see(&hit_pos.to_vec3(), &random_dir) {
						break;
					}
					random_dir = random::<vec3>();
				}
				self.falling_sprites.push(FallingSprite {
					spr: spr,
					dir: random_dir,
				});
				voxlap.set_sphere(&hit_pos, size, voxlap::Remove);
			}
			if create_new_plasma {
				new_plasmas.push(*plasma);
			}
			if destruct_plasma {
				self.free_plasmas = self.free_plasmas + 1;
				plasma.free = true;
				continue;
			}
		}
		return new_plasmas;
	}

	pub fn draw_falling_sprites(&mut self, voxlap_renderer: &voxlap::RenderContext) {
		for falling_sprite in self.falling_sprites.iter() {
			voxlap_renderer.draw_sprite(&falling_sprite.spr);
		}
	}

	fn update_falling_sprites(&mut self) {
		let mut removing_indices = vec![];
		for (i, falling_sprite) in self.falling_sprites.iter_mut().enumerate() {
			falling_sprite.spr.add_pos(&falling_sprite.dir);
			if falling_sprite.spr.get_pos().z > 120f32 {
				removing_indices.push(i);
				//voxlap.set_kv6_into_vxl_memory(&falling_sprite.spr, voxlap::Insert);
				continue;
			}
			falling_sprite.dir = falling_sprite.dir + vec3::new(0f32, 0f32, 0.01f32);
		}
		for i in removing_indices.iter() {
			self.falling_sprites.remove(*i);
		}
	}

	fn handle_new_plasmas(&mut self, new_plasmas: &Vec<Plasma>) {
		for plasma in new_plasmas.iter() {
			let level = match plasma.typ {
				Multi(l) => l,
				_ => 1,
			};
			if level <= 1 {
				continue;
			}
			for _ in range(0, 5i32) {
				self.add_plasma(&plasma.pos, &(random::<vec3>()*2f32), 0, Multi(level/2));
			}
		}
	}

	pub fn draw_plasmas(&self, voxlap_renderer: &voxlap::RenderContext) {
		for plasma in self.plasmas.iter() {
			if plasma.free {
				continue;
			}
			let color = match plasma.typ {
				Single(_) => voxlap::Color::rgb(0x99, 0, 0x99),
				Multi(_) => random::<voxlap::Color>(),
				Rapid => voxlap::Color::rgb(255, 255, 255),
				Bomb => voxlap::Color::rgb(255, 0, 0),
			};
			let size = plasma.get_size();
			for k in  range_step_inclusive(16i, 0, -1) {
				let f = k as f32;
				let radius = ((16f32 - f)).sqrt() *(f*f) * 0.004f32 * (size as f32);
				let pos_modifier_vec = plasma.dir*((f-8f32)*-0.25f32*(size as f32));
				voxlap_renderer.draw_sphere_with_z_buffer(&(plasma.pos - pos_modifier_vec), radius, color);
			}
		}
	}
}
