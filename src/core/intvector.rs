
use std::ops;
use num_traits::{PrimInt, Signed, Zero};

#[derive(Clone, Copy)]
pub struct IntVector2<T: PrimInt> {
    pub x: T,
    pub y: T,
}

impl<T: PrimInt> IntVector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: PrimInt + Signed> IntVector2<T> {
    pub fn rotate_around_cell(&self, cell: Self, rotation: CardinalRotation) -> Self {
        let relative = *self - cell;

        let rotated_relative = match rotation {
            CardinalRotation::Rotate0 => relative,
            CardinalRotation::Rotate90 => Self { x: -relative.y, y: relative.x },
            CardinalRotation::Rotate180 => Self { x: -relative.x, y: -relative.y },
            CardinalRotation::Rotate270 => Self { x: relative.y, y: -relative.x },
        };

        cell + rotated_relative
    }

    pub fn rotate_around_corner(&self, corner: Self, rotation: CardinalRotation) -> Self {
        let relative = *self - corner;

        let rotated_relative = match rotation {
            CardinalRotation::Rotate0 => relative,
            CardinalRotation::Rotate90 => Self { x: -relative.y - T::one(), y: relative.x },
            CardinalRotation::Rotate180 => Self { x: -relative.x - T::one(), y: -relative.y - T::one() },
            CardinalRotation::Rotate270 => Self { x: relative.y, y: -relative.x - T::one() },
        };

        corner + rotated_relative
    }
}

impl<T: PrimInt> ops::Add for IntVector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: PrimInt> ops::Sub for IntVector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: PrimInt> Zero for IntVector2<T> {
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

#[derive(Clone, Copy)]
pub enum CardinalRotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

impl CardinalRotation {
    pub fn rotated_right(&self) -> Self {
        match *self {
            CardinalRotation::Rotate0 => CardinalRotation::Rotate270,
            CardinalRotation::Rotate90 => CardinalRotation::Rotate0,
            CardinalRotation::Rotate180 => CardinalRotation::Rotate90,
            CardinalRotation::Rotate270 => CardinalRotation::Rotate180,
        }
    }

    pub fn rotated_left(&self) -> Self {
        match *self {
            CardinalRotation::Rotate0 => CardinalRotation::Rotate90,
            CardinalRotation::Rotate90 => CardinalRotation::Rotate180,
            CardinalRotation::Rotate180 => CardinalRotation::Rotate270,
            CardinalRotation::Rotate270 => CardinalRotation::Rotate0,
        }
    }
}