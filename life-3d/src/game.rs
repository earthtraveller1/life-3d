// The file for the logic behind the game of life.

use crate::{math::Vec3, renderer::Renderer};

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
    cells: CellsArray,
}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        GameOfLife {
            cells: vec![[[Cell::Dead; ARENA_SIZE]; ARENA_SIZE]; ARENA_SIZE],
        }
    }

    fn clamp_coords(x: i32) -> usize {
        let arena_max_index = ARENA_SIZE - 1;

        if x < 0 {
            ((arena_max_index as i32) + x).try_into().unwrap()
        } else if x > arena_max_index as i32 {
            (x - (arena_max_index as i32)).try_into().unwrap()
        } else {
            x.try_into().unwrap()
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

                    let neighbour_x = Self::clamp_coords(cell_x as i32 + x_offset);
                    let neighbour_y = Self::clamp_coords(cell_y as i32 + y_offset);
                    let neighbour_z = Self::clamp_coords(cell_z as i32 + z_offset);

                    if self.cell(neighbour_x, neighbour_y, neighbour_z).is_alive() {
                        neighbours_count += 1;
                    }
                }
            }
        }

        neighbours_count
    }

    pub fn update_game(&mut self) {
        let mut new_cells = vec![[[Cell::Dead; ARENA_SIZE]; ARENA_SIZE]; ARENA_SIZE];

        for (y, layer) in self.cells().iter().enumerate() {
            for (x, row) in layer.iter().enumerate() {
                for (z, cell) in row.iter().enumerate() {
                    let live_neighbours = self.living_neighbours(x, y, z);
                    let new_cell = &mut new_cells[y][x][z];

                    if cell.is_alive() {
                        if live_neighbours < 2 {
                            *new_cell = Cell::Dead;
                        } else if live_neighbours == 2 || live_neighbours == 3 {
                            *new_cell = Cell::Alive;
                        } else if live_neighbours > 3 {
                            *new_cell = Cell::Dead;
                        }
                    } else {
                        if live_neighbours == 3 {
                            *new_cell = Cell::Alive;
                        } else {
                            *new_cell = Cell::Dead;
                        }
                    }
                }
            }
        }

        self.cells = new_cells;
    }

    pub fn render(&self, renderer: &mut Renderer, cell_size: f32) {
        renderer.remove_all_instances();

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

                        renderer.add_instance(Vec3::new(x, y, z));
                    })
            })
        });

        renderer.render();
    }

    pub fn cells(&self) -> &CellsArray {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut CellsArray {
        &mut self.cells
    }

    pub fn cell(&self, x: usize, y: usize, z: usize) -> Cell {
        self.cells()[y][x][z]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, z: usize, cell: Cell) {
        self.cells_mut()[y][x][z] = cell;
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
