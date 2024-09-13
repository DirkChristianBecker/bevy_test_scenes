//! This example demonstrates how to use the rotating shapes test scene.
extern crate bevy_test_scenes;
use bevy::prelude::*;
use crate::bevy_test_scenes::prelude::scenes::*;


fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
		.add_plugins(MovingShapes::default())
		.add_systems(Startup, setup)
		.run();
}

fn setup(
	mut commands: Commands,
) {

	// This is how you initialize the camera
	let translation = Vec3::new(0.0, 6.0, 12.0);
	let look_at = Vec3::new(0., 1., 0.);

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation)
                .looking_at(look_at, Vec3::Y),
            ..Default::default()
        },
    ));
}