use sdl3::{
	image::LoadTexture,
	render::{Texture, TextureCreator},
	video::WindowContext,
};

pub struct Assets<'a> {
	pub ball_texture: Texture<'a>,
}

impl<'a> Assets<'a> {
	pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
		Self {
			ball_texture: texture_creator
				.load_texture("assets/ball.png")
				.unwrap(),
		}
	}
}
