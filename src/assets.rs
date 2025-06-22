use std::collections::HashMap;
use std::borrow::Cow;
use macroquad::prelude::*;

pub struct Assets {
	pub textures: HashMap<String, Texture2D>,
	pub animations: HashMap<String, Animation>,
}

impl Assets {
	pub fn new() -> Self {
		Self {
			textures: HashMap::new(),
			animations: HashMap::new(),
		}
	}

	pub async fn load_tex(&mut self, name: &str) -> &Texture2D {
		self.textures.insert(name.to_string(), load_texture(&format!("assets/textures/{}.png", name)).await.expect("Texture not found"));
		&self.textures[name]
	}

	pub async fn load_animation(&mut self, name: &str) -> &Animation {
		let mut anim = Animation {
			frames: 0,
			index: 0,
			frame_duration: 0,
			name: name.to_string(),
		};

		let mut i = 0;
		loop {
			let filename = format!("assets/textures/{}{}.png", name, i);
			if let Ok(tex) = load_texture(&filename).await {
				self.textures.insert(name.to_string(), tex);
				anim.frames += 1;
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
	pub index: usize,
	pub frame_duration: usize,
	pub name: String,
}

impl Animation {
	pub fn update(&mut self) {
		self.index += 1;
		if self.index / self.frame_duration >= self.frames - 1 {
			self.index = 0;
		}
	}

	pub fn current_frame(&self) -> String {
		format!("{}{}", self.name, self.index / self.frame_duration)
	}

	pub fn get_size(&self) -> Vec2 {
		todo!("Not implimented")
	}
}