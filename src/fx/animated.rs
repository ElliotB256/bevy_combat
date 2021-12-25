//! Helper functions for creating fire-and-forget special effect animations like explosions and bullet flares.

use bevy::prelude::*;

struct AnimatedEffectPrefabs {
    small_explosion: AnimatedEffectData,
    small_muzzle_flare: AnimatedEffectData,
    medium_explosion: AnimatedEffectData,
    blue_laser_beam: AnimatedEffectData,
    green_laser_beam: AnimatedEffectData,
    tiny_plus_explosion: AnimatedEffectData,
    smoke1: AnimatedEffectData
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
    };

    commands.insert_resource(resources);
}


pub struct AnimatedEffectsPlugin;

impl Plugin for AnimatedEffectsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(create_animated.system());
        app.add_system(update_animated.system());
        app.add_startup_system(setup.system());
    }
}

/// Component that indicates an explosion should be created at the location of the entity.
pub struct CreateAnimatedEffect {
    pub transform: Transform,
    pub effect: AnimatedEffects,
    pub parent: Option<Entity>
}

#[derive(Clone, Copy)]
pub enum AnimatedEffects {
    SmallExplosion,
    MuzzleFlare,
    MediumExplosion,
    BlueLaserBeam,
    GreenLaserBeam,
    TinyPlusExplosion,
    Smoke1
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

fn update_animated(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (entity, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            if sprite.index as usize == texture_atlas.textures.len() {
                // if we are at the final frame, delete the explosion
                commands.entity(entity).despawn_recursive();
            } else {
                // Otherwise advance the explosion frames.
                sprite.index += 1;
            }
        }
    }
}

fn create_animated(
    mut commands: Commands,
    prefabs: Res<AnimatedEffectPrefabs>,
    query: Query<(Entity, &CreateAnimatedEffect)>,
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
            AnimatedEffects::Smoke1 => &prefabs.smoke1
        };

        // Spawn an effect
        let spawned = commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: prefab.atlas.clone(),
                transform: effect.transform,
                ..Default::default()
            })
            .insert(Timer::from_seconds(prefab.frame_time, true))
            .id();

        // if we have a parent add them.
        //if let Ok(parent) = parent_query.get_component::<Parent>(entity) {
        //    commands.entity(spawned).insert(parent.clone());
        //}
        if let Some(parent) = effect.parent {
            commands.entity(parent).push_children(&[spawned]);
        }
    }
}
