use macroquad::prelude::*;

use crate::entity::*;
use crate::event::*;
use crate::level::*;
use crate::locals::*;
use crate::assets::*;

pub struct Player {
	pub rect: Rect,
	pub vel: Vec2,

	// MovementSystem
	pub grounded: bool,

	// It's fine to store this in Player because there will only ever be one Player, so might as well
	pub tex: Texture2D,
	pub flipped: bool,
}

impl Player {
	pub const JUMP_VEL: f32 = 13.;
	pub const MOVE_DAMP: f32 = 1.1;

	pub unsafe fn new(pos: Vec2, player_tex: Texture2D, entities: Entities) -> *mut Box<Player> {
		let mut s = Box::new(Self {
			rect: Rect::new(pos.x, pos.y, player_tex.width(), player_tex.height()),
			vel: vec2(0., 0.),

			grounded: false,

			tex: player_tex,
			flipped: false,
		});

		let ptr = &mut s as *mut Box<Player>;

		(*entities).push(s);

		ptr
	}
}

impl Entity for Player {
	unsafe fn update(&mut self, level: &mut Level, entities: Entities, events: &mut Vec<Event>) {
		if self.grounded {
			self.vel.y = 0.;
			if is_key_down(KeyCode::Z) || is_key_down(KeyCode::Space) {
				self.vel.y -= Self::JUMP_VEL;
			}
		} else {
			self.vel.y += GRAVITY;
		}

		self.grounded = false;
		let d_rect = Rect::new(self.rect.x + 1., self.rect.y + self.vel.y, self.rect.w - 2., self.rect.h);
		for hitbox in level.hitboxes.iter() {
			if hitbox.flags & HitboxFlags::PlayerCollides as u16 == 0 {
				continue;
			}

        	if self.vel.y > 0. && self.rect.y + self.rect.h <= hitbox.rect.y {
        		if d_rect.overlaps(&hitbox.rect) {
        			self.vel.y = 0.;
        			self.grounded = true;
        			break;
        		}
        	}
        }

		let mut can_move_left = true;
        let mut can_move_right = true;
        let d_rect = Rect::new(self.rect.x + self.vel.x, self.rect.y, self.rect.w, self.rect.h);
        for hitbox in level.hitboxes.iter() {
        	if hitbox.flags & HitboxFlags::PlayerCollides as u16 == 0 {
				continue;
			}

        	if d_rect.overlaps(&hitbox.rect) {
        		// if it is too far below it gets skipped
        		if hitbox.rect.y > self.rect.y + self.rect.h {
        			continue;
        		}

        		// get above collision and snap accordingly
    			if hitbox.rect.left() <= self.rect.x && self.rect.x + self.rect.w <= hitbox.rect.right() {
	        		self.vel.y = 0.;
	        		self.rect.y = hitbox.rect.bottom() + 1.;
	        		continue;
	        	}

	        	// get left and right collision and snap accordingly
    			if self.vel.x > 0. {
    				self.rect.x = hitbox.rect.left() - self.rect.w;
	    			can_move_right = false;
	        	} else if self.vel.x < 0. {
	        		self.rect.x = hitbox.rect.right();
	        		can_move_left = false;
	        	}
    			self.vel.x = 0.;
    		}
        }

        if can_move_left {
        	if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        		self.flipped = true;
        		self.vel.x -= 0.5;
        		self.rect.x -= 1.;
        	}
        }
        if can_move_right {
        	if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        		self.flipped = false;
        		self.vel.x += 0.5;
        		self.rect.x += 1.;
        	}
        }

		self.rect.x += self.vel.x;
		self.rect.y += self.vel.y;

        self.vel.x /= Self::MOVE_DAMP;
	}

	unsafe fn draw(&self, assets: &Assets) {
		draw_texture_ex(
			&self.tex,
			self.rect.x,
			self.rect.y,
			WHITE,
			DrawTextureParams {
				flip_x: self.flipped,
				..Default::default()
			}
		);
	}
}