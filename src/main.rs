use bevy::prelude::*;
use std::collections::HashSet;

mod animation_handler;
mod timeline;

use crate::animation_handler::*;
use crate::timeline::*;

const TIMELINE_WIDTH: f32 = 300.0;
const TIMELINE_HEIGHT: f32 = 800.0;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
	let timeline = Timeline(vec![
		AnimationNode {
			pos: Vec2::new(0.0, 0.0),
			handle: AnimationNodeHandle::HandleFixed(Vec2::new(0.75, 0.0)),
		},
		AnimationNode {
			pos: Vec2::new(1.0, 100.0),
			handle: AnimationNodeHandle::HandleFixed(Vec2::new(-0.75, 0.0)),
		},
		AnimationNode {
			pos: Vec2::new(2.0, 200.0),
			handle: AnimationNodeHandle::HandleFixed(Vec2::new(0.25, 0.25)),
		},
		AnimationNode {
			pos: Vec2::new(3.0, 300.0),
			handle: AnimationNodeHandle::HandleFixed(Vec2::new(0.25, 0.1)),
		},
		AnimationNode {
			pos: Vec2::new(4.0, 0.0),
			handle: AnimationNodeHandle::HandleFixed(Vec2::new(0.25, 0.1)),
		},
	]);

	let node_mat = materials.add(Color::rgb(1.0, 0.0, 0.0).into());
	let handle_mat = materials.add(Color::rgb(0.0, 1.0, 0.0).into());
	let dot_mat = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
	let bg_mat = materials.add(Color::rgb(0.0, 0.0, 0.0).into());

	commands
		.spawn(Camera2dComponents::default())
		.spawn(UiCameraComponents::default())
		.spawn(SpriteComponents {
			sprite: Sprite {
				size: Vec2::new(100.0, 100.0),
			},
			..Default::default()
		})
		.with(AnimationHandler {
			timeline: timeline.clone(),
			..Default::default()
		})
		.spawn(NodeComponents {
			style: Style {
				position: Rect {
					bottom: Val::Px(10.0),
					left: Val::Px(10.0),
					..Default::default()
				},
				size: Size::new(Val::Px(TIMELINE_WIDTH), Val::Px(TIMELINE_HEIGHT)),
				..Default::default()
			},
			material: bg_mat,
			..Default::default()
		})
		.with_children(|parent| {
			let mut pixel_amt: usize = 0;
			let r = timeline.width() / TIMELINE_WIDTH;
			let r_big = TIMELINE_WIDTH / timeline.width();
			for i in 0..(TIMELINE_WIDTH as usize) {
				let pos_y = timeline.value(r * i as f32);
				let pos_x = i as f32;
				parent.spawn(NodeComponents {
					style: Style {
						size: Size::new(Val::Px(1.0), Val::Px(1.0)),
						position_type: PositionType::Absolute,
						position: Rect {
							bottom: Val::Px(pos_y),
							left: Val::Px(pos_x),
							..Default::default()
						},
						..Default::default()
					},
					material: dot_mat,
					..Default::default()
				});
				pixel_amt += 1;
			}
			println!("Added {} pixels", pixel_amt);
			for node in timeline.sorted() {
				parent.spawn(NodeComponents {
					style: Style {
						size: Size::new(Val::Px(3.0), Val::Px(3.0)),
						position_type: PositionType::Absolute,
						position: Rect {
							bottom: Val::Px(node.pos.y()),
							left: Val::Px(r_big * node.pos.x()),
							..Default::default()
						},
						..Default::default()
					},
					material: node_mat,
					..Default::default()
				});
				let mut spawn = |node: &AnimationNode, hpos: Option<Vec2>| {
					if let Some(handle) = hpos {
						parent.spawn(NodeComponents {
							style: Style {
								size: Size::new(Val::Px(3.0), Val::Px(3.0)),
								position_type: PositionType::Absolute,
								position: Rect {
									bottom: Val::Px(node.pos.y() + r_big * handle.y()),
									left: Val::Px(r_big * (node.pos.x() + handle.x())),
									..Default::default()
								},
								..Default::default()
							},
							material: handle_mat,
							..Default::default()
						});
					}
				};
				spawn(node, node.handle.handle_left());
				spawn(node, node.handle.handle_right());
			}
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
