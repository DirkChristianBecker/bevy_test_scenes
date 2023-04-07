use bevy::{
	prelude::*,
};

#[derive(Resource)]
pub struct Rotator {
    pub speed : f32,
    pub axis : Vec3,
}

impl Default for Rotator {
	fn default() -> Self {
		Rotator {
			speed: 0.25,
            axis : Vec3::Z,
		}
	}
}