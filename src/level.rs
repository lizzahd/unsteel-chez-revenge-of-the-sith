use std::io::{self, BufRead};
use std::fs::File;
use macroquad::prelude::*;

pub fn read_lines(path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load_map(path: &str) -> Vec<LevelHitbox> {
    let mut ret = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok) {
            let split = line.split(" ").collect::<Vec<&str>>();
            ret.push(LevelHitbox {
                rect: Rect::new(
                	split[0].parse::<f32>().expect("Uh oh, problem parsing string. Oh no!"),
	                split[1].parse::<f32>().expect("Uh oh, problem parsing string. Oh no!"),
	                split[2].parse::<f32>().expect("Uh oh, problem parsing string. Oh no!"),
	                split[3].parse::<f32>().expect("Uh oh, problem parsing string. Oh no!")
	            ),
                flags: split[4].parse::<u16>().expect("Uh oh, problem parsing string. Oh no!"),
            });
        }
    } else {
        println!("Error: File \"{}\" not found", path);
    }

    ret
}

#[repr(u16)]
pub enum HitboxFlags {
	PlayerCollides = 1,
}

pub struct LevelHitbox {
	pub rect: Rect,
	pub flags: u16,
}

pub struct Level {
	pub hitboxes: Vec<LevelHitbox>,
	pub fg_tex: Texture2D,
}

impl Level {
	pub async fn load(name: &str) -> Self {
		Self {
			hitboxes: load_map(&format!("levels/{}/{}_hitboxes", name, name)),
			fg_tex: load_texture(&format!("levels/{}/{}_fg.png", name, name)).await.expect("Map file not found"),
		}
	}

	pub fn draw(&self) {
		draw_texture(&self.fg_tex, 0., 0., WHITE);

		// for rect in self.hitboxes.iter() {
		// 	draw_rectangle_lines(rect.rect.x, rect.rect.y, rect.rect.w, rect.rect.h, 2., BLUE);
		// }
	}
}