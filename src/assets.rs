use std::collections::HashMap;
use std::borrow::Cow;
use macroquad::prelude::*;
use macroquad::audio::{Sound, load_sound};

pub struct Assets {
	pub textures: HashMap<String, Texture2D>,
	pub animations: HashMap<String, Animation>,
	pub sounds: HashMap<String, Sound>,
}

impl Assets {
	pub fn new() -> Self {
		Self {
			textures: HashMap::new(),
			animations: HashMap::new(),
			sounds: HashMap::new(),
		}
	}

	pub async fn load_tex(&mut self, name: &str) -> &Texture2D {
		self.textures.insert(name.to_string(), load_texture(&format!("assets/textures/{}.png", name)).await.expect("Texture not found"));
		&self.textures[name]
	}

	pub async fn load_sound(&mut self, name: &str) -> &Sound {
		self.sounds.insert(name.to_string(), load_sound(&format!("assets/sounds/{}.png", name)).await.expect("Texture not found"));
		&self.sounds[name]
	}

	pub async fn load_sounds(&mut self, name: &str) {
		let mut i = 0;
		loop {
			let f_name = format!("{}{}", name, i);
			let filename = format!("assets/sounds/{}.wav", f_name);
			if let Ok(sound) = load_sound(&filename).await {
				self.sounds.insert(f_name.to_string(), sound);
			} else {
				break;
			}

			i += 1;
		}
	}

	pub async fn load_animation(&mut self, name: &str) -> &Animation {
		let mut anim = Animation {
			frames: 0,
			frame: 0,
			loop_start: 0,

			index: 0,
			frame_duration: 1,
			name: name.to_string(),
			size: vec2(0., 0.),
		};

		let mut i = 0;
		loop {
			let f_name = format!("{}{}", name, i);
			let filename = format!("assets/textures/{}.png", f_name);
			if let Ok(tex) = load_texture(&filename).await {
				anim.size.x = tex.width();
				anim.size.y = tex.height();
				anim.frames += 1;

				self.textures.insert(f_name, tex);
			} else {
				break;
			}

			i += 1;
		}

		self.animations.insert(name.to_string(), anim);
		&self.animations[name]
	}
}

#[derive(Clone)]
pub struct Animation {
	pub frames: usize,
	pub frame: usize,
	pub loop_start: usize,

	pub index: usize,
	pub frame_duration: usize,
	pub name: String,
	pub size: Vec2,
}

impl Animation {
	pub fn update(&mut self) {
		self.index += 1;
		self.frame = self.index / self.frame_duration;
		if self.frame >= self.frames - 1 {
			self.index = self.loop_start * self.frame_duration;
		}
	}

	pub fn current_frame(&self) -> String {
		format!("{}{}", self.name, self.frame)
	}
}