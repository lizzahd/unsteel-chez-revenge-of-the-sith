use crate::event::*;
use crate::level::*;
use crate::assets::*;

pub type Entities = *mut Vec<Box<dyn Entity>>;

pub trait Entity {
	unsafe fn update(&mut self, level: &mut Level, entities: Entities, events: &mut Vec<Event>);
	unsafe fn draw(&self, assets: &Assets);
}