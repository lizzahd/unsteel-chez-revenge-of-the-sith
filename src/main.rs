use macroquad::prelude::*;

use crate::entity::*;
use crate::player::*;

mod entity;
mod event;
mod player;

fn conf() -> Conf {
    Conf {
        window_width: 1152,
        window_height: 648,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    unsafe {
        let mut entities = Vec::<Box<dyn Entity>>::new();
        let entities_ptr = &mut entities as Entities;

        let player = Player::new(entities_ptr);
        
        loop {
            clear_background(BLACK);

            next_frame().await;
        }
    }
}
