use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::distributions::uniform::SampleRange;
use rand_chacha::{
    ChaCha8Rng,
    rand_core::{RngCore, SeedableRng},
};

use crate::{constants::STAR_LENGTH, states::GameStates};

pub fn plugin(app: &mut App) {
    app.init_resource::<StarRng>()
        .add_systems(OnEnter(GameStates::Playing), scene_setup_system)
        .add_systems(
            Update,
            (
                star_spawn_system.run_if(on_timer(Duration::from_millis(250))),
                star_move_system,
                star_despawn_system,
            )
                .run_if(in_state(GameStates::Playing)),
        );
}

#[derive(Component, Debug, Default)]
pub struct Star {
    pub word: String,
}

impl Star {
    fn new(word: &str) -> Self {
        Self { word: word.into() }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct StarRng(ChaCha8Rng);

impl Default for StarRng {
    fn default() -> Self {
        Self(ChaCha8Rng::seed_from_u64(19878367467712))
    }
}

fn scene_setup_system(mut commands: Commands) {
    commands.spawn((PointLight::default(),));
}

fn star_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<StarRng>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.into_inner();
    let Some(spawn_position) = get_spawn_position(camera, camera_transform, 3.0, 8.0, &mut rng)
    else {
        return;
    };

    commands.spawn((
        Star::new("egg"),
        Mesh3d(meshes.add(Cuboid::from_length(STAR_LENGTH))),
        MeshMaterial3d(materials.add(Color::hsl(
            ((rng.next_u32() % 180 + 165) % 360) as f32,
            0.3,
            0.4,
        ))),
        Transform::from_translation(spawn_position),
    ));
}

fn star_move_system(time: Res<Time>, mut stars: Query<&mut Transform, With<Star>>) {
    for mut star in &mut stars {
        star.translation.y -= time.delta_secs() * 0.5;
        star.rotate_y(time.delta_secs());
        star.rotate_x(time.delta_secs() * 0.4);
    }
}

fn star_despawn_system(
    mut commands: Commands,
    mut stars: Query<(Entity, &mut Transform), With<Star>>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.into_inner();
    let Some(lowest_visible_y) = get_lowest_visible_y(camera, camera_transform, 10.0) else {
        return;
    };

    for (entity, transform) in &mut stars {
        if transform.translation.y < lowest_visible_y {
            commands.entity(entity).despawn();
        }
    }
}

fn get_spawn_position(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    near_depth: f32,
    far_depth: f32,
    rng: &mut StarRng,
) -> Option<Vec3> {
    let viewport_size = camera.logical_viewport_size()?;

    let top_left = camera
        .viewport_to_world(camera_transform, Vec2::new(0.0, 0.0))
        .ok()?;

    let top_right = camera
        .viewport_to_world(camera_transform, Vec2::new(viewport_size.x, 0.0))
        .ok()?;

    let z = (near_depth..far_depth).sample_single(&mut rng.0);

    let top_left_distance =
        top_left.intersect_plane(Vec3::new(0., 0., -z), InfinitePlane3d::new(Vec3::Z))?;
    let top_right_distance =
        top_left.intersect_plane(Vec3::new(0., 0., -z), InfinitePlane3d::new(Vec3::Z))?;

    let top_left_at_z = top_left.get_point(top_left_distance);
    let top_right_at_z = top_right.get_point(top_right_distance);

    let x = (top_left_at_z.x..top_right_at_z.x).sample_single(&mut rng.0);
    let y = top_left_at_z.y + STAR_LENGTH;

    Some(Vec3::new(x, y, -z))
}

fn get_lowest_visible_y(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    far_depth: f32,
) -> Option<f32> {
    let viewport_size = camera.logical_viewport_size()?;

    let bottom_left = camera
        .viewport_to_world(camera_transform, Vec2::new(0.0, viewport_size.y))
        .ok()?;

    let bottom_left_distance = bottom_left
        .intersect_plane(Vec3::new(0., 0., -far_depth), InfinitePlane3d::new(Vec3::Z))?;

    let bottom_left_at_depth = bottom_left.get_point(bottom_left_distance);

    Some(bottom_left_at_depth.y)
}
