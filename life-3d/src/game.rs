// The file for the logic behind the game of life.

use crate::{
    math::{Mat4, Vec3},
    renderer::Renderer,
    shader_program_from_resources,
    shaders::{self, ShaderProgram},
};

#[derive(Clone, Copy)]
pub enum Cell {
    Alive,
    Dead,
}

pub struct Cursor {
    x: u32,
    y: u32,
    z: u32,

    shader_program: ShaderProgram,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            x: ARENA_SIZE as u32 / 2,
            y: ARENA_SIZE as u32 / 2,
            z: ARENA_SIZE as u32 / 2,
            shader_program: shader_program_from_resources!(
                shaders::CURSOR_VERT,
                shaders::CURSOR_FRAG
            ),
        }
    }

    pub fn move_x(&mut self, dx: i32) {
        self.x = ((self.x as i32) + dx) as u32
    }

    pub fn move_y(&mut self, dy: i32) {
        self.y = ((self.y as i32) + dy) as u32
    }

    pub fn move_z(&mut self, dz: i32) {
        self.z = ((self.z as i32) + dz) as u32
    }

    pub fn render(
        &self,
        game: &GameOfLife,
        renderer: &Renderer,
        cell_size: f32,
        projection: &Mat4,
        view: &Mat4,
    ) {
        let program = self.shader_program.use_program();

        program.set_uniform(
            "model",
            Mat4::translate(
                GameOfLife::to_real_coords(self.x as f32, cell_size),
                GameOfLife::to_real_coords(self.y as f32, cell_size),
                GameOfLife::to_real_coords(self.z as f32, cell_size),
            ),
        );
        program.set_uniform("view", view);
        program.set_uniform("projection", projection);

        if game
            .cell(
                self.x.try_into().unwrap(),
                self.y.try_into().unwrap(),
                self.z.try_into().unwrap(),
            )
            .is_alive()
        {
            program.set_uniform("in_color", Vec3::new(1.0, 1.0, 0.0));
        } else {
            program.set_uniform("in_color", Vec3::new(0.0, 1.0, 0.0));
        }

        renderer.render_one(false);
    }
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

pub const ARENA_SIZE: usize = 128;
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
                        if live_neighbours < 3 {
                            *new_cell = Cell::Dead;
                        } else if live_neighbours == 3 || live_neighbours == 5 {
                            *new_cell = Cell::Alive;
                        } else if live_neighbours > 5 {
                            *new_cell = Cell::Dead;
                        }
                    } else {
                        if live_neighbours == 5 {
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

    pub fn to_real_coords(x: f32, cell_size: f32) -> f32 {
        (x as f32 * cell_size) - ((ARENA_SIZE / 2) as f32) * cell_size
    }

    pub fn render(&self, renderer: &mut Renderer, cell_size: f32, cursor: &Cursor) {
        renderer.remove_all_instances();

        self.cells().iter().enumerate().for_each(|(y, layer)| {
            layer.iter().enumerate().for_each(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, cell)| cell.is_alive())
                    .for_each(|(z, _)| {
                        if x != cursor.x as usize
                            || y != cursor.y as usize
                            || z != cursor.z as usize
                        {
                            let (x, y, z) = (
                                Self::to_real_coords(x as f32, cell_size),
                                Self::to_real_coords(y as f32, cell_size),
                                Self::to_real_coords(z as f32, cell_size),
                            );

                            renderer.add_instance(Vec3::new(x, y, z));
                        }
                    })
            })
        });

        renderer.render_many();
    }
    
    pub fn flip_at_cursor(&mut self, cursor: &Cursor) {
        self.set_cell(
            cursor.x as usize,
            cursor.y as usize,
            cursor.z as usize,
            if self.cell(cursor.x as usize, cursor.y as usize, cursor.z as usize).is_alive() {
                Cell::Dead
            } else {
                Cell::Alive
            },
        );
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
