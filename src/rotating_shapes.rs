use bevy::{
	prelude::*,
	render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy::input::keyboard::KeyboardInput;
use bevy::app::AppExit;
use bevy::input::ButtonState;


use crate::rotator::Rotator;

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.;
use std::f32::consts::PI;

#[derive(Component, Default)]
pub struct RotatingShapes { }

impl Plugin for RotatingShapes {
	fn build(&self, app: &mut App) {
		app.insert_resource(Rotator::default());
		app.add_systems(Startup, setup);
		app.add_systems(Update, rotate);
		app.add_systems(Update, handle_keyboard);
	}
}

/// Quit the application if the user pressed escape.
fn handle_keyboard(
    mut key_evr: EventReader<KeyboardInput>,
	mut exit: EventWriter<AppExit>
) {
    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                
            },
            ButtonState::Released => {
				if ev.key_code == Some(KeyCode::Escape) {
					exit.send(AppExit);
				} 
			}
        }
    }
}

/// Spawn some shapes 
pub fn setup(
	mut commands: Commands,
	mut windows: Query<&mut Window>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut images: ResMut<Assets<Image>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// Adjust window title
	let mut window = windows.single_mut();
	window.title = String::from("bevy example scene");

	// The rest is just stuff to look at.
	let debug_material = materials.add(StandardMaterial {
		base_color_texture: Some(images.add(uv_debug_texture())),
		..default()
	});

	let shapes = [
		meshes.add(shape::Cube::default().into()),
		meshes.add(shape::Box::default().into()),
		meshes.add(shape::Capsule::default().into()),
		meshes.add(shape::Torus::default().into()),
		meshes.add(shape::Cylinder::default().into()),
		meshes.add(shape::UVSphere::default().into()),
	];

	let num_shapes = shapes.len();

	for (i, shape) in shapes.into_iter().enumerate() {
		commands.spawn((
			PbrBundle {
				mesh: shape,
				material: debug_material.clone(),
				transform: Transform::from_xyz(
					-X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
					2.0,
					0.0,
				)
				.with_rotation(Quat::from_rotation_x(-PI / 4.)),
				..default()
			},
			Shape,
		));
	}

	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 9000.0,
			range: 100.,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(8.0, 16.0, 8.0),
		..default()
	});

	// ground plane
	commands.spawn(PbrBundle {
		mesh: meshes.add(
			shape::Plane {
				size: 50.,
				subdivisions: 1,
			}
			.into(),
		),
		material: materials.add(Color::SILVER.into()),
		..default()
	});
}

/// Rotate the shape using the given settings
fn rotate(
	mut query: Query<&mut Transform, With<Shape>>, 
	rotator : Res<Rotator>, 
	time: Res<Time>) {
	for mut transform in &mut query {
		transform.rotate_axis(rotator.axis, time.delta_seconds() * rotator.speed);
	}
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
	const TEXTURE_SIZE: usize = 8;

	let mut palette: [u8; 32] = [
		255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255, 198, 255, 102, 198,
		255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
	];

	let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
	for y in 0..TEXTURE_SIZE {
		let offset = TEXTURE_SIZE * y * 4;
		texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
		palette.rotate_right(4);
	}

	Image::new_fill(
		Extent3d {
			width: TEXTURE_SIZE as u32,
			height: TEXTURE_SIZE as u32,
			depth_or_array_layers: 1,
		},
		TextureDimension::D2,
		&texture_data,
		TextureFormat::Rgba8UnormSrgb,
	)
}