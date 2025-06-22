#![allow(warnings)]

use std::collections::HashMap;
use macroquad::prelude::*;

use crate::entity::*;
use crate::assets::*;
use crate::player::*;
use crate::event::*;
use crate::level::*;

mod entity;
mod event;
mod player;
mod level;
mod locals;
mod projectiles;
mod assets;

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
        let mut assets = Assets::new();
        let mut animations = HashMap::<String, Animation>::new();

        assets.load_animation("fard").await;

        let mut level = Level::load("level0").await;

        let mut entities = Vec::<Box<dyn Entity>>::new();
        let entities_ptr = &mut entities as Entities;

        let mut events = Vec::<Event>::new();

        Player::new(vec2(100., 0.), load_texture("assets/textures/assu_chan.png").await.unwrap(), entities_ptr);

        loop {
            clear_background(BLACK);

            level.draw();

            let mut i = 0;
            while i < events.len() {
                if events[i].execute(entities_ptr, &assets) {
                    events.remove(i);
                    continue;
                }

                i += 1;
            }

            i = 0;
            while i < entities.len() {
                entities[i].update(&mut level, entities_ptr, &mut events);
                entities[i].draw(&assets);

                i += 1;
            }

            next_frame().await;
        }
    }
}
