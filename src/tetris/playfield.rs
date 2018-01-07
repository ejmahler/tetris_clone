use ndarray::{Array, Array2, Axis};

use super::tetromino::{Tetromino, OrphanBlock};
use ::engine::intvector::IntVector2;

const PLAYFIELD_WIDTH: usize = 10;
const PLAYFIELD_HEIGHT: usize = 22;
const PLAYFIELD_VISIBLE_HEIGHT: usize = 20;

pub struct Playfield {
    cells: Array2<Option<OrphanBlock>>,
}

impl Playfield {
    pub fn new_empty() -> Self {
        Self {
            cells: Array::default((PLAYFIELD_WIDTH,PLAYFIELD_HEIGHT)),
        }
    }

    pub fn spawn_location(&self) -> IntVector2<i8> {
        IntVector2::new(PLAYFIELD_WIDTH as i8 / 2, PLAYFIELD_HEIGHT as i8 - 2)
    }

    #[allow(unused)]
    pub fn visible_dimensions(&self) -> (usize, usize) {
        (PLAYFIELD_WIDTH, PLAYFIELD_VISIBLE_HEIGHT)
    }

    pub fn is_valid_placement(&self, tetromino: &Tetromino) -> bool {
        for cell in &tetromino.get_occupied_cells() {

            // The tetromino can't go here if it has negative coordinates
            if cell.x < 0 || cell.y < 0 {
                return false;
            }

            // The tetromino can't go here if its coordinates are bigger than the dimensions of the board
            let coordinates = [cell.x as usize, cell.y as usize];
            for i in 0..2 {
                if coordinates[i] >= self.cells.len_of(Axis(i)) {
                    return false;
                }
            }

            // The tetromino can't go here if this cell is already occupied by an orphaned block
            if let Some(_) = self.cells[coordinates] {
                return false;
            }
        }

        true
    }

    pub fn lock_tetromino(&mut self, tetromino: &Tetromino) {
        let color = tetromino.get_color();

        for cell in &tetromino.get_occupied_cells() {
            let coordinates = [cell.x as usize, cell.y as usize];
            self.cells[coordinates] = Some(OrphanBlock { color });
        }
    }

    pub fn iter_orphans(&self) -> impl Iterator<Item=(IntVector2<i8>, &OrphanBlock)> {
        self.cells.indexed_iter().filter_map(|(coordinates, element)| {
            if let &Some(ref block) = element {
                Some((IntVector2::new(coordinates.0 as i8, coordinates.1 as i8), block))
            }
            else {
                None
            }
        })
    }
}