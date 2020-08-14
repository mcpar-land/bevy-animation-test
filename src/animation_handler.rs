use crate::timeline::*;
use bevy::prelude::*;

pub enum AnimationType {
	Single,
	Loop,
	LoopPingPong,
}

#[derive(Bundle)]
pub struct AnimationHandler {
	pub timeline: Timeline,
	pub running: bool,
	pub position: f32,
	pub value: f32,
	pub speed: f32,
	pub animation_type: AnimationType,
	pub pong: bool,
}

impl Default for AnimationHandler {
	fn default() -> Self {
		Self {
			timeline: Timeline(vec![]),
			running: true,
			position: 0.0,
			value: 0.0,
			speed: 1.0,
			animation_type: AnimationType::Loop,
			pong: false,
		}
	}
}

fn animation_system(time: Res<Time>, mut query: Query<&mut AnimationHandler>) {
	for mut handler in &mut query.iter() {
		if !handler.running {
			continue;
		}
		handler.position += time.delta_seconds
			* handler.speed
			* if handler.pong { -1.0 } else { 1.0 };

		if handler.position > handler.timeline.right_bound()
			|| handler.position < 0.0
		{
			match handler.animation_type {
				AnimationType::Single => {
					handler.position =
						handler.timeline.value(handler.timeline.right_bound());
					handler.running = false;
				}
				AnimationType::Loop => {
					handler.position = 0.0;
				}
				AnimationType::LoopPingPong => {
					if handler.pong {
						handler.position = 0.0;
					} else {
						handler.position =
							handler.timeline.value(handler.timeline.right_bound());
					}
					handler.pong = !handler.pong;
				}
			};
		}
		handler.value = handler.timeline.value(handler.position);
	}
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
	fn build(&self, builder: &mut AppBuilder) {
		builder.add_system(animation_system.system());
	}
}
