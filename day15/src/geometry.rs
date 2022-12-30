#![allow(dead_code)]

use std::ops::{Add, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const ALL_UNIT_VECTORS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    pub const fn unit_vector(&self) -> Vector2D {
        let (x, y) = match self {
            Direction::Up => Self::ALL_UNIT_VECTORS[0],
            Direction::Down => Self::ALL_UNIT_VECTORS[1],
            Direction::Left => Self::ALL_UNIT_VECTORS[2],
            Direction::Right => Self::ALL_UNIT_VECTORS[3],
        };
        Vector2D::new(x, y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector2D {
    pub x: i64,
    pub y: i64,
}

impl Vector2D {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub const fn manhattan_length(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Neg for Vector2D {
    type Output = Vector2D;

    fn neg(self) -> Self::Output {
        Vector2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        self + (-rhs)
    }
}
