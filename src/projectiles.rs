use macroquad::prelude::*;

use crate::entity::*;
use crate::event::*;
use crate::level::*;
use crate::assets::*;

// Preload assets of projectiles based on the player's loadout
pub struct Projectile {
	pub rect: Rect,
	pub vel: Vec2,

	pub anim: Animation,
}

impl Projectile {
	pub unsafe fn new(pos: Vec2, vel: Vec2, anim: Animation, entities: Entities) -> *mut Box<Self> {
		let size = anim.get_size();
		let mut s = Box::new(Self {
			rect: Rect::new(pos.x, pos.y, size.x, size.y),
			vel,
			anim,
		});

		let ptr = &mut s as *mut Box<Self>;

		(*entities).push(s);

		ptr
	}
}

impl Entity for Projectile {
	unsafe fn update(&mut self, level: &mut Level, entities: Entities, events: &mut Vec<Event>) {
		self.anim.update();
	}

	unsafe fn draw(&self, assets: &Assets) {
		draw_texture_ex(&assets.textures[&self.anim.current_frame()], self.rect.x, self.rect.y, WHITE, DrawTextureParams {
			..Default::default()
		});
	}
}