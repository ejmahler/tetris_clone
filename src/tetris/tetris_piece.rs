
use image::Rgb;
use rand::Rng;

use num_traits::Zero;

use ::core::intvector::{IntVector2, CardinalRotation};

#[derive(Clone, Copy)]
pub enum PieceType {
    IBlock,
    OBlock,
    LBlock,
    RBlock,
    SBlock,
    ZBlock,
    TBlock,
}

impl PieceType {
    pub fn get_color(&self) -> Rgb<u8> {
        match *self {
            PieceType::IBlock => Rgb([240,10,10]),
            PieceType::OBlock => Rgb([10,240,240]),
            PieceType::LBlock => Rgb([240,120,10]),
            PieceType::RBlock => Rgb([240,10,240]),
            PieceType::SBlock => Rgb([10,240,10]),
            PieceType::ZBlock => Rgb([10,10,240]),
            PieceType::TBlock => Rgb([220,220,220]),
        }
    }
}

#[derive(Clone, Copy)]
pub struct TetrisPiece {
    piece_type: PieceType,
    orientation: CardinalRotation,
    position: IntVector2<i8>,
}

impl TetrisPiece {
    pub fn new_random(position: IntVector2<i8>, rng: &mut impl Rng) -> Self {
        Self {
            piece_type: *rng.choose(&[PieceType::IBlock, PieceType::LBlock, PieceType::RBlock, PieceType::SBlock, PieceType::ZBlock, PieceType::TBlock, PieceType::OBlock]).unwrap(),
            orientation: CardinalRotation::Rotate0,
            position,
        }
    }

    pub fn get_occupied_cells(&self) -> [IntVector2<i8>; 4] {
        let (rotate_around_corner, mut cells) = match self.piece_type {
            PieceType::OBlock => return [IntVector2::new(0,0), IntVector2::new(0,1), IntVector2::new(1,0), IntVector2::new(1,1)],
            
            PieceType::IBlock => (true, [IntVector2::new(-1,0), IntVector2::new(0,0), IntVector2::new(1,0), IntVector2::new(2,0)]),
            PieceType::LBlock => (false, [IntVector2::new(-1,0), IntVector2::new(0,0), IntVector2::new(1,0), IntVector2::new(1,1)]),
            PieceType::RBlock => (false, [IntVector2::new(-1,1), IntVector2::new(-1,0), IntVector2::new(0,0), IntVector2::new(1,0)]),
            PieceType::SBlock => (false, [IntVector2::new(-1,0), IntVector2::new(0,0), IntVector2::new(0,1), IntVector2::new(1,1)]),
            PieceType::ZBlock => (false, [IntVector2::new(-1,1), IntVector2::new(0,1), IntVector2::new(0,0), IntVector2::new(1,0)]),
            PieceType::TBlock => (false, [IntVector2::new(-1,0), IntVector2::new(0,0), IntVector2::new(1,0), IntVector2::new(0,1)]),
        };

        if rotate_around_corner {
            for entry in &mut cells {
                *entry = self.position + entry.rotate_around_corner(IntVector2::new(1,0), self.orientation);
            }
        }
        else {
            for entry in &mut cells {
                *entry = self.position + entry.rotate_around_cell(IntVector2::zero(), self.orientation);
            }
        }

        cells
    }

    pub fn rotated_right(&self) -> Self {
        Self {
            piece_type: self.piece_type,
            orientation: self.orientation.rotated_right(),
            position: self.position,
        }
    }

    pub fn rotated_left(&self) -> Self {
        Self {
            piece_type: self.piece_type,
            orientation: self.orientation.rotated_left(),
            position: self.position,
        }
    }

    pub fn get_color(&self) -> Rgb<u8> {
        self.piece_type.get_color()
    }
}
