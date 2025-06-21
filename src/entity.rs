use crate::event::*;

pub type Entities = *mut Vec<Box<dyn Entity>>;

pub trait Entity {
	unsafe fn update(&mut self, entities: Entities, events: &mut Vec<Event>);
	unsafe fn draw(&self);
}

#[macro_export]
macro_rules! impl_entity {
	{
		$s_name:ident;
		update $s_update:block;
		draw $s_draw:block;
	} => {
		impl Entity for $s_name {
			unsafe fn update(&mut self, entities: Entities, events: &mut Vec<Event>) $s_update
			unsafe fn draw(&self) $s_draw
		}
	}
}