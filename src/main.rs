use ggez::{ContextBuilder, conf::{WindowMode}, event::{self, EventHandler}, GameResult};
use ggez::graphics::{self, Mesh, DrawMode, Rect, Color};
use ggez::error::GameError;
use ggez::timer;
use rand::Rng;

const CELL_SIZE: (f32, f32) = (20.0, 20.0);
const GRID_SIZE: (f32, f32) = (40.0, 40.0); 
const WINDOW_SIZE: (f32, f32) = (CELL_SIZE.0 * GRID_SIZE.0, CELL_SIZE.1 * GRID_SIZE.1);

struct State {
    grid: Vec<Vec<bool>>,
    fps: u32,
    running: bool,
}

impl State {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut grid = vec![];

        for _ in 0..GRID_SIZE.0 as usize {
            let row: Vec<bool> = (0..GRID_SIZE.1 as usize).map(|_| rng.gen::<bool>()).collect();
            grid.push(row);
        }

        State {
            grid,
            fps: 1,
            running: true,
        }
    }
}


impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        while timer::check_update_time(ctx, self.fps) && self.running {
            let mut coords: Vec<(usize, usize)> = vec![];

            for i in 0..GRID_SIZE.0 as usize {

                let left = if i > 0 { i - 1 } else { GRID_SIZE.0 as usize - 1 }; 
                let right = if i < GRID_SIZE.0 as usize - 1 { i + 1 } else { 0 };

                for j in 0..GRID_SIZE.1 as usize {
                    let up = if j > 0 { j - 1 } else { GRID_SIZE.1 as usize - 1 }; 
                    let down = if j < GRID_SIZE.1 as usize - 1 { j + 1 } else { 0 };    

                    let neighbours = self.grid[left][j] as u8 
                                    + self.grid[left][up] as u8 
                                    + self.grid[i][up] as u8 
                                    + self.grid[right][up] as u8 
                                    + self.grid[right][j] as u8 
                                    + self.grid[right][down] as u8 
                                    + self.grid[i][down] as u8 
                                    + self.grid[left][down] as u8;

                    if self.grid[i][j] && (neighbours < 2 || neighbours > 3) {
                        coords.push((i, j));
                    }
                    else if !self.grid[i][j] && neighbours == 3 {
                        coords.push((i, j));
                    }
                }
            }

            for coord in coords {
                self.grid[coord.0][coord.1] ^= true;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);

        for i in 0..GRID_SIZE.0 as usize {
            for j in 0..GRID_SIZE.1 as usize {
                if self.grid[i][j] {
                    let rect = Mesh::new_rectangle(
                        ctx,
                        DrawMode::fill(),
                        Rect::new(i as f32 * CELL_SIZE.0, j as f32 * CELL_SIZE.1, CELL_SIZE.0, CELL_SIZE.1),
                        Color::BLACK,
                    )?;
                    graphics::draw(ctx, &rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
                }
            }

            if i == 0 {
                continue;
            }

            let line = Mesh::new_line(
                ctx,
                &vec![
                    ggez::mint::Point2 { 
                        x: i as f32 * CELL_SIZE.0, 
                        y: 0.0 
                    },
                    ggez::mint::Point2 { 
                        x: i as f32 * CELL_SIZE.0, 
                        y: WINDOW_SIZE.1 
                    },
                ]
                .as_slice(),
                2.0,
                Color::BLACK,
            )?;

            graphics::draw(ctx, &line, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self, 
        _ctx: &mut ggez::Context, 
        _button: ggez::event::MouseButton, 
        x: f32, 
        y: f32
    ) {
        self.grid[(x / CELL_SIZE.0) as usize][(y / CELL_SIZE.1) as usize] ^= true;
    }

    fn key_down_event(
        &mut self, 
        _ctx: &mut ggez::Context, 
        keycode: ggez::event::KeyCode, 
        _keymods: ggez::event::KeyMods, 
        repeat: bool
    ) {
        if keycode == event::KeyCode::Space && !repeat {
            self.running ^= true;
        }

        if keycode == event::KeyCode::Up && !repeat {
            self.fps += 1;
        }

        if keycode == event::KeyCode::Down && !repeat {
            self.fps -= 1;
        }

        if keycode == event::KeyCode::R && !repeat {
            self.grid = vec![vec![false; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize];
        }
    }
}

fn main() -> GameResult<()> {
    let mut state = State::new();

    // state.grid[5][6] = true;
    // state.grid[6][6] = true;
    // state.grid[6][7] = true;
    // state.grid[6][8] = true;

    let (ctx, event_loop) = ContextBuilder::new("Game of Life", "Me")
        .window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
        .build()?;
        
    event::run(ctx, event_loop, state);
    Ok(())
}
