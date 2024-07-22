// The file for the logic behind the game of life.

use crate::{math::Mat4, renderer::Renderer, shaders::UsedShaderProgram};

#[derive(Clone, Copy)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        match *self {
            Cell::Alive => true,
            Cell::Dead => false,
        }
    }

    pub fn is_dead(&self) -> bool {
        !self.is_alive()
    }
}

const ARENA_SIZE: usize = 16;
type CellsArray = Vec<[[Cell; ARENA_SIZE]; ARENA_SIZE]>;

pub struct GameOfLife {
    cells_1: CellsArray,
    cells_2: CellsArray,

    using_cells_1: bool,
}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        GameOfLife {
            cells_1: vec![[[Cell::Dead; ARENA_SIZE]; ARENA_SIZE]; ARENA_SIZE],
            cells_2: vec![[[Cell::Dead; ARENA_SIZE]; ARENA_SIZE]; ARENA_SIZE],
            using_cells_1: true,
        }
    }

    pub fn living_neighbours(&self, cell_x: usize, cell_y: usize, cell_z: usize) -> u32 {
        let mut neighbours_count = 0;

        for y_offset in -1..=1 as i32 {
            for x_offset in -1..=1 as i32 {
                for z_offset in -1..=1 as i32 {
                    if y_offset == 0 && x_offset == 0 && z_offset == 0 {
                        continue;
                    }

                    let neighbour_x = ((cell_x as i32 + x_offset) % ARENA_SIZE as i32) as usize;
                    let neighbour_y = ((cell_y as i32 + y_offset) % ARENA_SIZE as i32) as usize;
                    let neighbour_z = ((cell_z as i32 + z_offset) % ARENA_SIZE as i32) as usize;

                    if self.cell(neighbour_x, neighbour_y, neighbour_z).is_alive() {
                        neighbours_count += 1;
                    }
                }
            }
        }

        neighbours_count
    }

    pub fn render(&self, renderer: &Renderer, shaders: &UsedShaderProgram, cell_size: f32) {
        self.cells().iter().enumerate().for_each(|(y, layer)| {
            layer.iter().enumerate().for_each(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, cell)| cell.is_alive())
                    .for_each(|(z, _)| {
                        let (x, y, z) = (
                            (x as f32 * cell_size) - ((ARENA_SIZE / 2) as f32) * cell_size,
                            (y as f32 * cell_size) - ((ARENA_SIZE / 2) as f32) * cell_size,
                            (z as f32 * cell_size) - ((ARENA_SIZE / 2) as f32) * cell_size,
                        );

                        let model = Mat4::translate(x, y, z);
                        shaders.set_uniform("model", &model);
                        renderer.render();
                    })
            })
        })
    }

    pub fn cells(&self) -> &CellsArray {
        if self.using_cells_1 {
            &self.cells_1
        } else {
            &self.cells_2
        }
    }

    pub fn back_cells(&self) -> &CellsArray {
        if self.using_cells_1 {
            &self.cells_2
        } else {
            &self.cells_1
        }
    }

    pub fn back_cells_mut(&mut self) -> &mut CellsArray {
        if self.using_cells_1 {
            &mut self.cells_2
        } else {
            &mut self.cells_1
        }
    }

    pub fn cells_mut(&mut self) -> &mut CellsArray {
        if self.using_cells_1 {
            &mut self.cells_1
        } else {
            &mut self.cells_2
        }
    }

    pub fn cell(&self, x: usize, y: usize, z: usize) -> Cell {
        self.cells()[y][x][z]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, z: usize, cell: Cell) {
        self.cells_mut()[y][x][z] = cell;
    }

    pub fn flip_buffers(&mut self) {
        self.using_cells_1 = !self.using_cells_1;
    }
}

#[cfg(test)]
mod tests {
    use super::{Cell, GameOfLife};

    #[test]
    fn neighbour_count_test() {
        let mut game = Box::new(GameOfLife::new());

        game.set_cell(3, 4, 3, Cell::Alive);
        game.set_cell(3, 4, 4, Cell::Alive);
        game.set_cell(2, 2, 3, Cell::Alive);

        assert_eq!(game.living_neighbours(3, 3, 3), 3);
    }
}
