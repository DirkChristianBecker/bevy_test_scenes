mod rotating_shapes;
mod rotator;
mod random_mover;
mod moving_shapes;

pub mod prelude {
	pub mod scenes {
		pub use crate::rotating_shapes::RotatingShapes;
		pub use crate::moving_shapes::MovingShapes;
	}

	pub mod tools {
		pub use crate::rotator::Rotator;
		pub use crate::random_mover::Mover;
	}
}
