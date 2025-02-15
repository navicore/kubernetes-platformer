use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::{color::palettes::tailwind::BLUE_600, prelude::*, sprite::Anchor, text::TextBounds};
use bevy_lit::prelude::{LightOccluder2d, PointLight2d};

use crate::room::room_component::RoomState;

use super::door_component::{Door, Platform, BOUNCE_EFFECT, PLATFORM_HEIGHT, PLATFORM_WIDTH};

#[allow(clippy::type_complexity)]
pub fn spawn_platforms(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    room_state: Res<RoomState>,
    query: Query<Entity, With<Platform>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !room_state.is_changed() {
        return;
    }

    debug!("Room state changed, respawning platforms...");

    despawn_existing_platforms(&mut commands, query);
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = create_text_font(font);

    for (position, room_name, room_id) in room_state.clone().doors.into_iter().map(|door_state| {
        (
            door_state.position,
            door_state.room_name,
            door_state.room_id,
        )
    }) {
        spawn_platform(
            &mut commands,
            position,
            &text_font,
            room_name,
            room_id,
            &mut meshes,
        );
    }
}

#[allow(clippy::type_complexity)]
fn despawn_existing_platforms(commands: &mut Commands, query: Query<Entity, With<Platform>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn create_text_font(font: Handle<Font>) -> TextFont {
    TextFont {
        font,
        font_size: 14.0,
        ..default()
    }
}

fn spawn_platform(
    commands: &mut Commands,
    position: Vec2,
    text_font: &TextFont,
    room_name: String,
    room_id: String,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let platform_shape = meshes.add(Rectangle::new(PLATFORM_WIDTH, PLATFORM_HEIGHT));

    let platform_component = (
        Mesh2d(platform_shape),
        LightOccluder2d::default(),
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            PLATFORM_WIDTH / 2.0,
            PLATFORM_HEIGHT / 2.0,
        )),
        Transform::from_xyz(position.x, position.y, 0.0),
        Friction {
            dynamic_coefficient: 0.6,
            static_coefficient: 0.8,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: BOUNCE_EFFECT,
            combine_rule: CoefficientCombine::Max,
        },
        Platform {},
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
            ..default()
        },
    );

    let text_component = (
        Text2d::new(room_name.clone()),
        text_font.clone(),
        Anchor::Center,
        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
        TextBounds::from(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
        Transform::from_translation(Vec3::Z),
    );

    let door_component = (
        Door { room_id, room_name },
        Transform::from_xyz(0.0, PLATFORM_HEIGHT / 2.0 + PLATFORM_WIDTH / 4.0, 0.0),
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(PLATFORM_WIDTH / 4.0, PLATFORM_WIDTH / 2.0)),
            ..default()
        },
    );
    let light_component = (
        PointLight2d {
            intensity: 1.5,
            radius: 600.0,
            falloff: 2.0,
            color: Color::from(BLUE_600),
            ..default()
        },
        Transform::from_xyz(0.0, PLATFORM_HEIGHT.mul_add(-2.0, PLATFORM_WIDTH), 0.0),
    );

    commands.spawn(platform_component).with_children(|builder| {
        builder.spawn(text_component);
        builder.spawn(door_component);
        builder.spawn(light_component);
    });
}
