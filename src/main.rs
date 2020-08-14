use bevy::prelude::*;
use std::collections::HashSet;

mod animation_handler;
mod timeline;

use crate::animation_handler::*;
use crate::timeline::*;

fn setup(mut commands: Commands) {
	commands
		.spawn(Camera2dComponents::default())
		.spawn(SpriteComponents {
			sprite: Sprite {
				size: Vec2::new(100.0, 100.0),
			},
			..Default::default()
		})
		.with(AnimationHandler {
			timeline: Timeline(vec![
				AnimationNode {
					pos: Vec2::new(0.0, 0.0),
					handle: AnimationNodeHandle::Flat,
				},
				AnimationNode {
					pos: Vec2::new(1.0, 100.0),
					handle: AnimationNodeHandle::Flat,
				},
				AnimationNode {
					pos: Vec2::new(2.0, 200.0),
					handle: AnimationNodeHandle::Flat,
				},
				AnimationNode {
					pos: Vec2::new(3.0, 300.0),
					handle: AnimationNodeHandle::Flat,
				},
				AnimationNode {
					pos: Vec2::new(4.0, 300.0),
					handle: AnimationNodeHandle::Flat,
				},
			]),
			..Default::default()
		});
}

fn animate_sprite_system(
	mut query: Query<(&AnimationHandler, &mut Translation)>,
) {
	for (handler, mut translation) in &mut query.iter() {
		*translation.x_mut() = handler.value;
	}
}

fn main() {
	App::build()
		.add_default_plugins()
		.add_plugin(AnimationPlugin)
		.add_startup_system(setup.system())
		.add_system(animate_sprite_system.system())
		.run();
}
