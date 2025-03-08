use bevy::{
	color::palettes::css::SILVER, 
    prelude::*, 
    render:: { 
        render_asset::RenderAssetUsages, 
        render_resource:: { Extent3d, TextureDimension, TextureFormat } 
    }
};
use bevy::input::keyboard::KeyboardInput;
use bevy::app::AppExit;
use bevy::input::ButtonState;

use crate::prelude::tools::*;

/// A marker to identify our shapes
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.;

#[derive(Component, Default)]
pub struct MovingShapes { }

impl Plugin for MovingShapes {
	fn build(&self, app: &mut App) {
        app.add_plugins(Mover::default());
		app.add_systems(Startup, setup);
		app.add_systems(Update, handle_keyboard);
	}
}

/// Quit the application if the user pressed escape.
fn handle_keyboard(
    mut key_evr: EventReader<KeyboardInput>,
	mut exit: EventWriter<AppExit>
) {
    for ev in key_evr.read() {
        match ev.state {
            ButtonState::Pressed => {
                
            },
            ButtonState::Released => {
				if ev.key_code == KeyCode::Escape {
					exit.send(AppExit::Success);
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
        meshes.add(Cuboid::default()),
        meshes.add(Tetrahedron::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Cone::default()),
        meshes.add(ConicalFrustum::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

	let num_shapes = shapes.len();
	for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(shape),
            MeshMaterial3d(debug_material.clone()),
            Mover::default(),
            Shape,
            Transform::from_xyz(
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                2.0,
                0.0,
            ))
        );
	}

	commands
        .spawn(Transform::from_xyz(8.0, 16.0, 8.0))
        .insert(PointLight{
            intensity: 10000000.0,
            range: 100.0,
            shadows_enabled: true,
            shadow_depth_bias: 0.2,
            ..default()
        });


	// ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
	const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
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
        RenderAssetUsages::RENDER_WORLD,
    )

}