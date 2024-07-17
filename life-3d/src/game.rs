// The file for the logic behind the game of life.

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

const ARENA_SIZE: usize = 128;
type CellsArray = [[[Cell; ARENA_SIZE]; ARENA_SIZE]; ARENA_SIZE];

pub struct GameOfLife {
    cells_1: CellsArray,
    cells_2: CellsArray,

    using_cells_1: bool,
}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        GameOfLife {
            cells_1: [[[Cell::Dead; ARENA_SIZE]; ARENA_SIZE]; ARENA_SIZE],
            cells_2: [[[Cell::Dead; ARENA_SIZE]; ARENA_SIZE]; ARENA_SIZE],
            using_cells_1: true,
        }
    }

    pub fn living_neighbours(&self, cell_x: usize, cell_y: usize, cell_z: usize) -> u32 {
        let mut neighbours_count = 0;

        for y_offset in -1..1 as i32 {
            for x_offset in -1..1 as i32 {
                for z_offset in -1..1 as i32 {
                    if y_offset == 0 && x_offset == 0 && z_offset == 0 {
                        continue;
                    }

                    let neighbour_x = ((cell_x as i32 + x_offset) % ARENA_SIZE as i32) as usize;
                    let neighbour_y  = ((cell_y as i32+ y_offset) % ARENA_SIZE as i32) as usize;
                    let neighbour_z = ((cell_z as i32 + z_offset) % ARENA_SIZE as i32) as usize;

                    if self.cell(neighbour_x, neighbour_y, neighbour_z).is_alive() {
                        neighbours_count += 1;
                    }
                }
            }
        }

        neighbours_count
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
