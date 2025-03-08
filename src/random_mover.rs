use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Mover {
    pub speed : f32,
	pub last_rotation_change : f32,
	pub last_distance : f32,
}

impl Default for Mover {
	fn default() -> Self {
		let mut rng = rand::rng();
    	let speed : f32 = rng.random_range(1.0..3.0);

		Mover {
			speed,
			last_rotation_change : 0.0,
			last_distance : 0.0,
		}
	}
}

impl Plugin for Mover {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, update);
	}
}

fn update(
    time: Res<Time>,
    mut q_movement: Query<(&mut Transform, &mut Mover)>,
) {
	let mut rng = rand::rng();
    for (mut transform, mut state) in &mut q_movement 
	{
		let state = &mut *state;

		let dist = transform.translation.distance_squared(Vec3::ZERO);
		if dist > 100.0  && dist >= state.last_distance {
			// You've wandered to far, turn around.
			transform.rotate_y(180.0);
		}
		state.last_distance = dist;
		
        let forward = transform.forward() * state.speed * time.delta().as_secs_f32();

    	transform.translation += forward;
		state.last_rotation_change += time.delta().as_secs_f32();
		if state.last_rotation_change > 0.25 {
			state.last_rotation_change = 0.0;
			transform.rotate_y(rng.random_range(-0.25 .. 0.25));
		}
	}
}