use bevy::prelude::*;
use std::f32::consts::PI;

use crate::game::components::*;
use crate::game::state::*;

const LOGO_SIZE: f32 = 80.0;
const LOGO_Y_OFFSET: f32 = 180.0;

pub fn spawn_logo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let logo_x = 0.0;
    let logo_y = LOGO_Y_OFFSET;

    // Create stylized X (left side)
    let x_offset = -LOGO_SIZE * 0.6;
    spawn_logo_x(
        &mut commands,
        Vec2::new(logo_x + x_offset, logo_y),
        &mut meshes,
        &mut materials,
    );

    // Create stylized O (right side)
    let o_offset = LOGO_SIZE * 0.6;
    spawn_logo_o(
        &mut commands,
        Vec2::new(logo_x + o_offset, logo_y),
        &mut meshes,
        &mut materials,
    );

    // Add rotating glow ring around the logo
    spawn_logo_glow(
        &mut commands,
        Vec2::new(logo_x, logo_y),
        &mut meshes,
        &mut materials,
    );
}

fn spawn_logo_x(
    commands: &mut Commands,
    center: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let size = LOGO_SIZE * 0.7;
    let thickness = 12.0;
    let z = 10.0;

    for angle in [45f32.to_radians(), -45f32.to_radians()] {
        // Main X line
        commands.spawn((
            Logo,
            Mesh2d(meshes.add(Rectangle::new(size, thickness))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(X_COLOR))),
            Transform::from_translation(Vec3::new(center.x, center.y, z))
                .with_rotation(Quat::from_rotation_z(angle)),
        ));

        // Glow layer
        commands.spawn((
            Logo,
            Mesh2d(meshes.add(Rectangle::new(size + 8.0, thickness + 8.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(X_GLOW))),
            Transform::from_translation(Vec3::new(center.x, center.y, z - 0.1))
                .with_rotation(Quat::from_rotation_z(angle)),
        ));
    }
}

fn spawn_logo_o(
    commands: &mut Commands,
    center: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let radius = LOGO_SIZE * 0.35;
    let thickness = 12.0;
    let z = 10.0;

    // Outer circle
    commands.spawn((
        Logo,
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(O_COLOR))),
        Transform::from_translation(Vec3::new(center.x, center.y, z)),
    ));

    // Inner circle (background color to create ring effect)
    commands.spawn((
        Logo,
        Mesh2d(meshes.add(Circle::new(radius - thickness))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(BG_COLOR))),
        Transform::from_translation(Vec3::new(center.x, center.y, z + 0.01)),
    ));

    // Outer glow
    commands.spawn((
        Logo,
        Mesh2d(meshes.add(Circle::new(radius + 4.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(O_GLOW))),
        Transform::from_translation(Vec3::new(center.x, center.y, z - 0.1)),
    ));
}

fn spawn_logo_glow(
    commands: &mut Commands,
    center: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let radius = LOGO_SIZE * 1.3;
    let z = 9.0;

    // Animated rotating ring with timer
    commands.spawn((
        Logo,
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgba(
            0.44, 1.0, 0.91, 0.08,
        )))),
        Transform::from_translation(Vec3::new(center.x, center.y, z)),
        LogoAnimation {
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            phase: 0.0,
        },
    ));
}

pub fn animate_logo(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut LogoAnimation), With<Logo>>,
) {
    for (mut transform, mut animation) in query.iter_mut() {
        animation.timer.tick(time.delta());
        animation.phase = animation.timer.fraction() * 2.0 * PI;

        // Gentle rotation
        transform.rotation = Quat::from_rotation_z(animation.phase * 0.2);

        // Subtle pulse effect
        let pulse = 1.0 + (animation.phase.sin() * 0.1);
        transform.scale = Vec3::splat(pulse);
    }
}

pub fn cleanup_logo(mut commands: Commands, query: Query<Entity, With<Logo>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
