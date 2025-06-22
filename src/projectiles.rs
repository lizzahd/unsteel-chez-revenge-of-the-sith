use macroquad::prelude::*;

use crate::entity::*;
use crate::event::*;
use crate::level::*;
use crate::assets::*;

// Preload assets of projectiles based on the player's loadout
pub struct Projectile {
	pub rect: Rect,
	pub vel: Vec2,

	pub target_mask: u16,
	pub damage: i32,

	pub lifetime: usize,

	pub anim: Animation,
	pub anim_spawn_end: usize,
}

impl Projectile {
	pub const LIFETIME: usize = 350;
	pub const VEL: f32 = 2.;

	pub unsafe fn new(pos: Vec2, vel: Vec2, anim: Animation, entities: Entities, target_mask: u16) -> *mut Box<Self> {
		let mut s = Box::new(Self {
			rect: Rect::new(pos.x, pos.y, anim.size.x, anim.size.y),
			vel: vel * Self::VEL,

			target_mask,
			damage: 5,

			lifetime: 0,

			anim,
			anim_spawn_end: 5,
		});

		s.anim.frame_duration = 10;

		let ptr = &mut s as *mut Box<Self>;

		(*entities).push(s);

		ptr
	}
}

impl Entity for Projectile {
	unsafe fn update(&mut self, level: &mut Level, entities: Entities, events: &mut Vec<Event>, assets: &Assets) -> u16 {
		self.lifetime += 1;

		if self.anim.loop_start == 0 {
			if self.anim.frame == self.anim_spawn_end {
				self.anim.loop_start = self.anim_spawn_end;
			}
		}

		if self.lifetime >= Self::LIFETIME {
			return EntityState::Dead as u16;
		}

		for entity in (*entities).iter_mut() {
			if entity.flags() & EntityFlag::Enemy as u16 != 0 {
				let e_rect = entity.get_rect();
				if e_rect.overlaps(&self.rect) {
					entity.apply_effect(Effect::Damage(self.damage));
				}
			}
		}

		self.rect.x += self.vel.x;
		self.rect.y += self.vel.y;

		self.anim.update();

		0
	}

	unsafe fn draw(&self, assets: &Assets) {
		let alpha = ((Self::LIFETIME as f32) - (self.lifetime as f32)) as f32 / (Self::LIFETIME as f32) * 255.;

		draw_texture_ex(&assets.textures[&self.anim.current_frame()], self.rect.x, self.rect.y, Color::from_rgba(255, 255, 255, alpha as u8), DrawTextureParams {
			..Default::default()
		});
	}

	unsafe fn flags(&self) -> u16 {
		EntityFlag::Projectile as u16
	}

	unsafe fn get_rect(&self) -> &Rect {
		&self.rect
	}

	unsafe fn apply_effect(&mut self, effect: Effect) {

	}
}