//! Helper functions for creating fire-and-forget special effect animations like explosions and bullet flares.

use std::time::Duration;

use bevy::prelude::*;

use crate::game::GameTimeDelta;

use super::beams::BeamTracking;

struct AnimatedEffectPrefabs {
    small_explosion: AnimatedEffectData,
    small_muzzle_flare: AnimatedEffectData,
    medium_explosion: AnimatedEffectData,
    blue_laser_beam: AnimatedEffectData,
    green_laser_beam: AnimatedEffectData,
    tiny_plus_explosion: AnimatedEffectData,
    smoke1: AnimatedEffectData,
    shield: AnimatedEffectData
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let resources = AnimatedEffectPrefabs {
        small_explosion: AnimatedEffectData::new(texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("art/small_explosion.png"),
            Vec2::new(16.0, 16.0),
            8,
            1,
        )),
        0.1),
        small_muzzle_flare: AnimatedEffectData::new(texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("art/muzzle_flare.png"),
            Vec2::new(8.0, 8.0),
            4,
            1,
        )),
        0.05),
        medium_explosion: AnimatedEffectData::new(texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("art/large_explosion.png"),
            Vec2::new(32.0, 32.0),
            9,
            1,
        )),
        0.1),
        blue_laser_beam: AnimatedEffectData::new(texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("art/laser_blue.png"),
            Vec2::new(4.0, 4.0),
            4,
            1,
        )),
        0.05),
        green_laser_beam: AnimatedEffectData::new(texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("art/laser_green.png"),
            Vec2::new(4.0, 4.0),
            4,
            1,
        )),
        0.05),
        tiny_plus_explosion: AnimatedEffectData::new(texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("art/tiny_plus_explosion.png"),
            Vec2::new(8.0, 8.0),
            5,
            1,
        )),
        0.05),
        smoke1: AnimatedEffectData::new(texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("art/smoke1.png"),
            Vec2::new(16.0, 16.0),
            12,
            1,
        )),
        0.2),
        shield: AnimatedEffectData::new(texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("art/shield2.png"),
            Vec2::new(64.0, 64.0),
            4,
            1,
        )),
        0.05),
    };

    commands.insert_resource(resources);
}


pub struct AnimatedEffectsPlugin;

impl Plugin for AnimatedEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_animated);
        app.add_system(update_animated);
        app.add_startup_system(setup);
    }
}

#[derive(Component)]
pub struct CreateAnimatedEffect {
    pub transform: Transform,
    pub effect: AnimatedEffects,
    pub parent: Option<Entity>
}

#[derive(Component)]
pub struct AnimatedEffect {
    pub finished: bool
}
impl AnimatedEffect {
    pub fn new() -> Self {
        AnimatedEffect { finished: false }
    }
}

#[derive(Clone, Copy)]
pub enum AnimatedEffects {
    SmallExplosion,
    MuzzleFlare,
    MediumExplosion,
    BlueLaserBeam,
    GreenLaserBeam,
    TinyPlusExplosion,
    Smoke1,
    Shield
}

struct AnimatedEffectData {
    atlas: Handle<TextureAtlas>,
    frame_time: f32
}

impl AnimatedEffectData {
    pub fn new(atlas: Handle<TextureAtlas>, frame_time: f32) -> Self {
        AnimatedEffectData { atlas, frame_time }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

fn update_animated(
    mut commands: Commands,
    time: Res<GameTimeDelta>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut AnimatedEffect,
        &Handle<TextureAtlas>,
    )>,
) {
    for (entity, mut timer, mut sprite, mut effect, texture_atlas_handle) in query.iter_mut() {
        timer.tick(Duration::from_secs_f32(time.0));
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            if effect.finished {
                commands.entity(entity).despawn_recursive();
            }

            if sprite.index < texture_atlas.textures.len() - 1 {
                //advance the frames.
                sprite.index += 1;
            } else {
                effect.finished = true;
            }
        }
    }
}

fn create_animated(
    mut commands: Commands,
    prefabs: Res<AnimatedEffectPrefabs>,
    query: Query<(Entity, &CreateAnimatedEffect)>,
    beam_track_query: Query<&BeamTracking>
) {
    for (entity, effect) in query.iter() {
        // despawn the creation command
        commands.entity(entity).despawn_recursive();

        let prefab = match effect.effect {
            AnimatedEffects::SmallExplosion => &prefabs.small_explosion,
            AnimatedEffects::MuzzleFlare => &prefabs.small_muzzle_flare,
            AnimatedEffects::MediumExplosion => &prefabs.medium_explosion,
            AnimatedEffects::BlueLaserBeam => &prefabs.blue_laser_beam,
            AnimatedEffects::GreenLaserBeam => &prefabs.green_laser_beam,
            AnimatedEffects::TinyPlusExplosion => &prefabs.tiny_plus_explosion,
            AnimatedEffects::Smoke1 => &prefabs.smoke1,
            AnimatedEffects::Shield => &prefabs.shield,
        };

        // Spawn an effect
        let spawned = commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: prefab.atlas.clone(),
                transform: effect.transform,
                ..Default::default()
            })
            .insert(AnimationTimer { 0: Timer::from_seconds(prefab.frame_time, true) })
            .insert(AnimatedEffect::new())
            .id();

        // if we have a parent add them.
        if let Some(parent) = effect.parent {
            commands.entity(parent).push_children(&[spawned]);
        }

        // hacky for now - add beam tracking if it exists
        if let Ok(beam_tracking) = beam_track_query.get_component::<BeamTracking>(entity) {
            commands.entity(spawned).insert(*beam_tracking);
        }
    }
}
