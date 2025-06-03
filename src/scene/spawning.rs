use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand_chacha::{
    ChaCha8Rng,
    rand_core::{RngCore, SeedableRng},
};

use crate::states::GameStates;

pub fn plugin(app: &mut App) {
    app.init_resource::<StarRng>()
        .add_systems(OnEnter(GameStates::Playing), scene_setup_system)
        .add_systems(
            Update,
            (
                star_spawn_system.run_if(on_timer(Duration::from_millis(1000))),
                star_move_system,
                star_despawn_system,
            ),
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
    let spawn_rect = get_visible_rectangle_at_depth(camera, camera_transform, 10.0).unwrap();

    commands.spawn((
        Star::new("egg"),
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::hsl(
            ((rng.next_u32() % 180 + 165) % 360) as f32,
            0.3,
            0.4,
        ))),
        Transform::from_translation(spawn_rect.sample_interior(&mut rng.0).extend(-10.0)),
    ));
}

fn star_move_system(time: Res<Time>, mut stars: Query<&mut Transform, With<Star>>) {
    for mut star in &mut stars {
        star.translation = star.translation.move_towards(Vec3::ZERO, time.delta_secs());
        star.rotate_y(time.delta_secs());
        star.rotate_x(time.delta_secs() * 0.4);
    }
}

fn star_despawn_system(
    mut commands: Commands,
    mut stars: Query<(Entity, &mut Transform), With<Star>>,
) {
    for (entity, transform) in &mut stars {
        if transform.translation.length() < 1.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn get_visible_rectangle_at_depth(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    depth: f32,
) -> Option<Rectangle> {
    // Get the viewport size
    let viewport_size = camera.logical_viewport_size()?;

    // Calculate the ray from screen corners
    let bottom_left = camera
        .viewport_to_world(camera_transform, Vec2::new(0.0, viewport_size.y))
        .ok()?;

    let top_right = camera
        .viewport_to_world(camera_transform, Vec2::new(viewport_size.x, 0.0))
        .ok()?;

    // Project rays to the desired depth plane
    let depth_ratio = depth / bottom_left.origin.distance(bottom_left.get_point(1.0));

    let bottom_left_at_depth = bottom_left.get_point(depth_ratio);
    let top_right_at_depth = top_right.get_point(depth_ratio);

    let rectangle = Rectangle::from_corners(top_right_at_depth.xy(), bottom_left_at_depth.xy());

    Some(rectangle)
}
