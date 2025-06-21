use macroquad::prelude::*;

use crate::entity::*;
use crate::event::*;

pub struct Player {

}

impl Player {
	pub unsafe fn new(entities: Entities) -> *mut Box<Player> {
		let mut s = Box::new(Self {

		});

		let ptr = &mut s as *mut Box<Player>;

		(*entities).push(s);

		ptr
	}
}

crate::impl_entity! {
	Player;
	update {

	};
	draw {

	};
}