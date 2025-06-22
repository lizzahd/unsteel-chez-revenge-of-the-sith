use std::collections::HashMap;
use macroquad::prelude::*;

use crate::entity::*;
use crate::projectiles::*;
use crate::assets::*;

pub enum EventType {
	SpawnProjectile {
		pos: Vec2,
		vel: Vec2,
		
		target_mask: u16,
	},
}

pub struct Event {
	pub event_type: EventType,
}

impl Event {
	pub unsafe fn execute(&mut self, entities: Entities, assets: &Assets) -> bool {
		match self.event_type {
			EventType::SpawnProjectile {pos, vel, target_mask, ..} => {
				Projectile::new(pos, vel, assets.animations["fard"].clone(), entities, target_mask);

				true
			}
			_ => true
		}
	}
}