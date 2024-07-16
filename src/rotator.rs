use bevy::prelude::*;

#[derive(Resource)]
pub struct Rotator {
    pub speed : f32,
    pub axis : Dir3,
}

impl Default for Rotator {
	fn default() -> Self {
		Rotator {
			speed: 0.25,
            axis : Dir3::Z,
		}
	}
}