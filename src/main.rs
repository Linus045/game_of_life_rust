use std::{fmt::Display, io::Stdout, thread::sleep, time::Duration};

use std::io::Write;

use terminal::{error, Action, Clear, Color, Terminal};

#[derive(Clone, Copy)]
enum CellState {
    Alive,
    Dead,
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Dead => write!(f, "Dead\n"),
            CellState::Alive => write!(f, "Alive\n"),
        }
    }
}

impl PartialEq for CellState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CellState::Dead, CellState::Dead) => true,
            (CellState::Alive, CellState::Alive) => true,
            _ => false,
        }
    }
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Alive
    }
}

const CELL_WIDTH: usize = 50;
const CELL_HEIGHT: usize = 50;

type CellArray = [[CellState; CELL_WIDTH]; CELL_HEIGHT];

struct Gamestate {
    pub cells: CellArray,
}

impl Gamestate {
    fn new() -> Box<Gamestate> {
        let mut state = Gamestate {
            cells: [[CellState::Alive; CELL_WIDTH]; CELL_HEIGHT],
        };

        for x in 0..CELL_WIDTH {
            for y in 0..CELL_HEIGHT {
                state.cells[x][y] = if x > CELL_WIDTH / 2 && y > 5 {
                    CellState::Dead
                } else {
                    CellState::Alive
                };
            }
        }
        Box::new(state)
    }
}

fn render_game(cells: &CellArray, term: &mut Terminal<Stdout>) {
    for x in 0..CELL_WIDTH {
        for y in 0..CELL_HEIGHT {
            term.batch(Action::MoveCursorTo(x as u16, y as u16))
                .expect("Failed to move cursor");
            match cells[x][y] {
                CellState::Dead => {
                    term.batch(Action::SetForegroundColor(Color::Black))
                        .expect("[Dead] Failed to set foreground color");
                    term.write(format!("{}", " ").as_bytes())
                        .expect("Failed to write");
                }
                CellState::Alive => {
                    term.batch(Action::SetForegroundColor(Color::White))
                        .expect("[Alive] Failed to set foreground color");
                    term.write(format!("{}", "A").as_bytes())
                        .expect("Failed to write");
                }
            };
        }
        println!();
    }
    term.flush_batch().expect("Failed to flush batch");
}

fn update_game(cells: &mut CellArray) {
    let old_cells = cells.clone();

    for x in 0..CELL_WIDTH {
        for y in 0..CELL_HEIGHT {
            let mut alive_neighbours = 0;
            for xoff in (-1i32..=1).rev() {
                for yoff in (-1i32..=1).rev() {
                    let neighbour_x = (x as i32) + xoff;
                    let neighbour_y = (y as i32) + yoff;
                    if neighbour_x < 0
                        || neighbour_y < 0
                        || neighbour_x >= CELL_WIDTH as i32
                        || neighbour_y >= CELL_HEIGHT as i32
                    {
                        continue;
                    }

                    if xoff == 0 && yoff == 0 {
                        continue;
                    }

                    if old_cells[neighbour_x as usize][neighbour_y as usize] == CellState::Alive {
                        alive_neighbours += 1;
                    }
                }
            }
            cells[x][y] = match (cells[x][y], alive_neighbours) {
                (CellState::Alive, 0) => CellState::Dead,
                (CellState::Alive, 1) => CellState::Dead,
                (CellState::Alive, 2) => CellState::Alive,
                (CellState::Alive, 3) => CellState::Alive,
                (CellState::Alive, _) => CellState::Dead,
                (CellState::Dead, 3) => CellState::Alive,
                (CellState::Dead, _) => CellState::Dead,
            }
        }
    }
}

fn main() -> error::Result<()> {
    let mut terminal = terminal::stdout();

    let mut game = Gamestate::new();
    terminal.act(Action::ClearTerminal(Clear::All))?;
    loop {
        update_game(&mut game.cells);
        render_game(&game.cells, &mut terminal);
        sleep(Duration::new(0, 20 * 1000 * 1000));
    }
}
