
use image::Rgb;
use rand::Rng;

use num_traits::Zero;

use ::engine::intvector::{IntVector2, CardinalRotation};

#[derive(Clone, Copy)]
pub enum PieceType {
    IBlock,
    OBlock,
    LBlock,
    JBlock,
    SBlock,
    ZBlock,
    TBlock,
}

impl PieceType {
    pub fn get_color(&self) -> Rgb<u8> {
        match *self {
            PieceType::IBlock => Rgb([10,220,220]),
            PieceType::OBlock => Rgb([210,210,10]),
            PieceType::LBlock => Rgb([240,120,10]),
            PieceType::JBlock => Rgb([10,20,250]),
            PieceType::SBlock => Rgb([10,240,10]),
            PieceType::ZBlock => Rgb([240,10,10]),
            PieceType::TBlock => Rgb([140,10,240]),
        }
    }
}

enum RotationType {
    None,
    AroundCorner,
    AroundCell,
}

#[derive(Clone, Copy)]
pub struct Tetromino {
    piece_type: PieceType,
    orientation: CardinalRotation,
    position: IntVector2<i8>,
}

impl Tetromino {
    pub fn new_random(position: IntVector2<i8>, rng: &mut impl Rng) -> Self {
        Self {
            piece_type: *rng.choose(&[PieceType::IBlock, PieceType::LBlock, PieceType::JBlock, PieceType::SBlock, PieceType::ZBlock, PieceType::TBlock, PieceType::OBlock]).unwrap(),
            orientation: CardinalRotation::Rotate0,
            position,
        }
    }

    pub fn get_occupied_cells(&self) -> [IntVector2<i8>; 4] {
        let (rotation_type, mut cells) = match self.piece_type {
            PieceType::OBlock => (RotationType::None, [IntVector2::new(0,0), IntVector2::new(0,1), IntVector2::new(1,0), IntVector2::new(1,1)]),
            
            PieceType::IBlock => (RotationType::AroundCorner, [IntVector2::new(-1,0), IntVector2::new(0,0), IntVector2::new(1,0), IntVector2::new(2,0)]),

            PieceType::LBlock => (RotationType::AroundCell, [IntVector2::new(-1,0), IntVector2::new(0,0), IntVector2::new(1,0), IntVector2::new(1,1)]),
            PieceType::JBlock => (RotationType::AroundCell, [IntVector2::new(-1,1), IntVector2::new(-1,0), IntVector2::new(0,0), IntVector2::new(1,0)]),
            PieceType::SBlock => (RotationType::AroundCell, [IntVector2::new(-1,0), IntVector2::new(0,0), IntVector2::new(0,1), IntVector2::new(1,1)]),
            PieceType::ZBlock => (RotationType::AroundCell, [IntVector2::new(-1,1), IntVector2::new(0,1), IntVector2::new(0,0), IntVector2::new(1,0)]),
            PieceType::TBlock => (RotationType::AroundCell, [IntVector2::new(-1,0), IntVector2::new(0,0), IntVector2::new(1,0), IntVector2::new(0,1)]),
        };

        match rotation_type {
            RotationType::None =>
                for entry in &mut cells {
                    *entry = self.position + *entry;
                },
            RotationType::AroundCorner =>
                for entry in &mut cells {
                    *entry = self.position + entry.rotate_around_corner(IntVector2::new(1,0), self.orientation);
                },
            RotationType::AroundCell =>
                for entry in &mut cells {
                    *entry = self.position + entry.rotate_around_cell(IntVector2::zero(), self.orientation);
                },
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

    pub fn moved(&self, offset: IntVector2<i8>) -> Self {
        Self {
            piece_type: self.piece_type,
            orientation: self.orientation,
            position: self.position + offset,
        }
    }

    pub fn get_color(&self) -> Rgb<u8> {
        self.piece_type.get_color()
    }
}

pub struct OrphanBlock {
    pub color: Rgb<u8>,
}
