use macroquad::prelude::*;

use crate::event::*;
use crate::level::*;
use crate::assets::*;

pub type Entities = *mut Vec<Box<dyn Entity>>;

#[repr(u16)]
pub enum EntityState {
	Dead		= 1 << 0,
}

#[repr(u16)]
pub enum EntityFlag {
	Player		= 1 << 0,
	Enemy		= 1 << 1,
	Projectile	= 1 << 2,
}

#[derive(Debug)]
pub enum Effect {
	Damage(i32),
}

pub trait Entity {
	unsafe fn update(&mut self, level: &mut Level, entities: Entities, events: &mut Vec<Event>, assets: &Assets) -> u16;
	unsafe fn draw(&self, assets: &Assets);

	unsafe fn flags(&self) -> u16;
	unsafe fn get_rect(&self) -> &Rect;

	unsafe fn apply_effect(&mut self, effect: Effect);
}