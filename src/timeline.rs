use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum AnimationNodeHandle {
	Flat,
	/// A handle with no curve on either side
	Straight,
	/// A handle with a curve node on the left side
	HandleLeft(Vec2),
	/// A handle with a curve node on the right side
	HandleRight(Vec2),
	/// A handle with a curve node on both sides
	HandleBoth(Vec2, Vec2),
	/// A handle with a fixed pair of curve nodes that are
	/// [equidistant](https://en.wikipedia.org/wiki/Equidistant) and
	/// [colinear](https://en.wikipedia.org/wiki/Collinearity)
	HandleFixed(Vec2),
}

impl AnimationNodeHandle {
	pub fn handle_left(&self) -> Option<Vec2> {
		match self {
			Self::Flat => None,
			Self::Straight => None,
			Self::HandleLeft(h) => Some(*h),
			Self::HandleRight(_) => None,
			Self::HandleBoth(h, _) => Some(*h),
			Self::HandleFixed(h) => Some(if h.x() <= 0.0 { *h } else { -*h }),
		}
	}
	pub fn handle_right(&self) -> Option<Vec2> {
		match self {
			Self::Flat => None,
			Self::Straight => None,
			Self::HandleLeft(_) => None,
			Self::HandleRight(h) => Some(*h),
			Self::HandleBoth(_, h) => Some(*h),
			Self::HandleFixed(h) => Some(if h.x() >= 0.0 { *h } else { -*h }),
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct AnimationNode {
	pub pos: Vec2,
	pub handle: AnimationNodeHandle,
}

#[derive(Debug)]
pub enum NodePositionQuery {
	ZeroNodes,
	Current(AnimationNode),
	Between(AnimationNode, AnimationNode),
	BeforeBounds(AnimationNode),
	AfterBounds(AnimationNode),
}

pub struct Timeline(pub Vec<AnimationNode>);

impl Timeline {
	pub fn sorted(&self) -> Vec<&AnimationNode> {
		let mut h: Vec<&AnimationNode> = self.0.iter().collect();
		h.sort_by(|a, b| a.pos.x().partial_cmp(&b.pos.x()).unwrap());
		h
	}
	pub fn sorted_mut(&mut self) -> Vec<&mut AnimationNode> {
		let mut h: Vec<&mut AnimationNode> = self.0.iter_mut().collect();
		h.sort_by(|a, b| a.pos.x().partial_cmp(&b.pos.x()).unwrap());
		h
	}
	pub fn first(&self) -> &AnimationNode {
		self.sorted()[0]
	}
	pub fn last(&self) -> &AnimationNode {
		self.sorted()[self.0.len() - 1]
	}
	pub fn nearest(&self, x: f32) -> NodePositionQuery {
		if self.0.len() == 0 {
			return NodePositionQuery::ZeroNodes;
		}

		let mut res: Option<NodePositionQuery> = None;

		let mut left: Option<AnimationNode> = None;

		for (i, node) in self.sorted().iter().enumerate() {
			if node.pos.x() == x {
				res = Some(NodePositionQuery::Current(**node));
			}
			if i == 0 && node.pos.x() > x {
				res = Some(NodePositionQuery::BeforeBounds(**node));
				break;
			}
			if i == self.0.len() - 1 && node.pos.x() < x {
				res = Some(NodePositionQuery::AfterBounds(**node));
				break;
			}
			if node.pos.x() < x {
				left = Some(**node);
			}
			if left.is_some() && node.pos.x() > x {
				res = Some(NodePositionQuery::Between(left.unwrap(), **node));
				break;
			}
		}

		res.unwrap()
	}

	/// A Vec of path segments between points, and the two possible handles affecting them
	pub fn segments(&self) -> Vec<(Option<Vec2>, Option<Vec2>)> {
		let mut res: Vec<(Option<Vec2>, Option<Vec2>)> = vec![];

		let sorted = self.sorted();

		res.push((None, sorted[0].handle.handle_left()));

		let mut left: AnimationNode = *sorted[0];

		for node in sorted.iter().skip(1) {
			res.push((left.handle.handle_right(), node.handle.handle_left()));
		}

		// res.push((None, sorted[self.0.len() - 1].handle.handle_right()));

		res
	}

	pub fn value(&self, x: f32) -> f32 {
		use NodePositionQuery::*;
		let query = self.nearest(x);
		// strange beizer magic
		// https://www.icode.com/c-function-for-a-bezier-curve/
		fn beizer(t: f32, p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2) -> f32 {
			let cx = 3.0 * (p1.x() - p0.x());
			let cy = 3.0 * (p1.y() - p0.y());
			let bx = 3.0 * (p2.x() - p1.x()) - cx;
			let by = 3.0 * (p2.y() - p1.y()) - cy;
			let ax = p3.x() - p0.x() - cx - bx;
			let ay = p3.y() - p0.y() - cy - by;

			let cube = t * t * t;
			let square = t * t;
			let res_x = (ax * cube) + (bx * square) + (cx * t) + p0.x();
			let res_y = (ay * cube) + (by * square) + (cy * t) + p0.y();

			res_y
		}

		match query {
			ZeroNodes => 0.0,
			Current(node) => node.pos.y(),
			BeforeBounds(node) => node.pos.y(),
			AfterBounds(node) => node.pos.y(),
			Between(a, b) => {
				if let AnimationNodeHandle::Flat = a.handle {
					a.pos.y()
				} else {
					match (a.handle.handle_right(), b.handle.handle_left()) {
						(None, None) => 0.0,
						(Some(left), Some(right)) => 0.0,
						(None, Some(right)) => 0.0,
						(Some(left), None) => 0.0,
					}
				}
			}
		}
	}

	/// The smallest value represented on this timeline
	pub fn lower_bound(&self) -> f32 {
		// TODO ??? mysterious beizer math goes here
		0.0
	}

	/// The largest value respresented on this timeline
	pub fn upper_bound(&self) -> f32 {
		// TODO ??? mysterious beizer math goes here
		0.0
	}

	pub fn left_bound(&self) -> f32 {
		self.first().pos.x()
	}

	pub fn right_bound(&self) -> f32 {
		self.last().pos.x()
	}
}
