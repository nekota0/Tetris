use std::process::exit;
use rand::seq::SliceRandom;
use std::error::Error;


pub const HEIGHT: u16 = 16;
pub const WIDTH: u16 = 14;

pub const MAX_HEIGTH: i32 = HEIGHT as i32 - 1;
pub const MAX_WIDTH: i32 = WIDTH as i32 - 1;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

pub enum Direction {
    Stop,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone)]
pub enum Block {
    L,
    J,
    I,
    O,
    S,
    Z, 
    T,
}


pub struct Game {
    pub position: Point,
    pub block_dir: i32,
    pub block_spec: Block,
    pub block_arr_list: Vec<Vec<Point>>,
    pub block_arr: Vec<Point>,
    pub direction: Direction,
    pub score: u16,
    pub pile: Vec<Point>
}


impl Game {
    pub fn new() -> Game {
        Game {
            position: Point { x: WIDTH as i32 / 2, y: 0},
            block_dir: 1,
            block_spec: Block::J,
            block_arr_list: vec![],
            block_arr: vec![],
            direction: Direction::Down,
            score: 0,
            pile: vec![]
        }
    }


    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.moving_pos();
        self.matching_pos_to_block();
        self.block_spining();
        self.prevent_move_to_crashing();
        self.prevent_spin_to_crashing();
        self.prevent_escaping();
        self.check_game_over();
        self.scoring();
        self.draw();
        
        Ok(())
    }


    pub fn moving_pos(&mut self) {
        let current_pos = &self.position;
        let new_pos = match self.direction {
            Direction::Stop => Point { x: current_pos.x, y: current_pos.y},
            Direction::Down => {
                if self.block_arr.iter().all(|pointer| pointer.y < MAX_HEIGTH) {
                    Point { x: current_pos.x, y: current_pos.y + 1}
                } else {
                    Point { x: current_pos.x, y: current_pos.y}
                }
            },
            Direction::Left => {
                if self.block_arr.iter().all(|pointer| pointer.x > 0) {
                    Point { x: current_pos.x - 1, y: current_pos.y}
                } else {
                    Point { x: current_pos.x, y: current_pos.y}
                }
            },
            Direction::Right => {
                if self.block_arr.iter().all(|pointer| pointer.x < MAX_WIDTH) {
                    Point { x: current_pos.x + 1, y: current_pos.y}
                } else {
                    Point { x: current_pos.x, y: current_pos.y}
                }
            },
        };

        self.position = new_pos;
    }


    pub async fn falling(&mut self) {
        let current_pos = &self.position;
        self.position = Point { x: current_pos.x, y: current_pos.y - 1 };
        self.draw();
    }


    pub fn draw(&self) {
        for y in 0..HEIGHT {
            print!("     <!");
            for x in 0..WIDTH {
                let point = Point { x: x as i32, y: y as i32};
                if self.block_arr.iter().any(|z| z == &point) || self.pile.iter().any(|z| z == &point) {
                    print!("@");
                } else {
                    print!(" ");
                }
            }
            print!("!>");

            println!();
        }

        println!("     <!{}!>", "=".repeat(WIDTH as usize));
        println!("       {}  ", r"\/".repeat(WIDTH as usize / 2));
        println!();
        println!("        Score: {} ", self.score);
        println!();
    }


    pub fn block_spining(&mut self) {
        match self.block_dir % 4 {
            1 => self.block_arr = self.block_arr_list[0].clone(),
            2 => self.block_arr = self.block_arr_list[1].clone(),
            3 => self.block_arr = self.block_arr_list[2].clone(),
            0 => self.block_arr = self.block_arr_list[3].clone(),
            _ => ()
        }
    }


    pub fn prevent_escaping(&mut self) {
        let current_pos = &self.position;

        if self.block_arr.iter().any(|pointer| pointer.x < 0) {
            self.position = Point { x: current_pos.x + 1, y: current_pos.y};
        } else if self.block_arr.iter().any(|pointer| pointer.x > MAX_WIDTH) {
            self.position = Point { x: current_pos.x - 1, y: current_pos.y};
        } else if self.block_arr.iter().any(|pointer| pointer.y > MAX_HEIGTH) {
            self.position = Point { x: current_pos.x, y: current_pos.y - 1};
        } else {
            ()
        }

        self.matching_pos_to_block();
        self.block_spining();
    }


    pub fn prevent_move_to_crashing(&mut self) {
        let current_pos = &self.position;
        let new_pos = match self.direction {
            Direction::Stop => {
                Point { x: current_pos.x, y: current_pos.y}
            },
            Direction::Down => {
                if self.block_arr.iter().any(|&pointer| self.pile.contains(&pointer)) {
                    Point { x: current_pos.x, y: current_pos.y - 1}
                } else {
                    Point { x: current_pos.x, y: current_pos.y}
                }
            },
            Direction::Left => {
                if self.block_arr.iter().any(|&pointer| self.pile.contains(&pointer)) {
                    Point { x: current_pos.x + 1, y: current_pos.y}
                } else {
                    Point { x: current_pos.x, y: current_pos.y}
                }
            },
            Direction::Right => {
                if self.block_arr.iter().any(|pointer| self.pile.contains(pointer)) {
                    Point { x: current_pos.x - 1, y: current_pos.y}
                } else {
                    Point { x: current_pos.x, y: current_pos.y}
                }
            },
        };

        self.position = new_pos;
        
        self.matching_pos_to_block();
        self.block_spining();
    }


    pub fn prevent_spin_to_crashing(&mut self) {
        loop {
            if self.block_arr.iter().any(|pointer| self.pile.contains(pointer)) {
                self.position.y -= 1;
                self.matching_pos_to_block();
                self.block_spining();
            } else {
                break
            }
        }
    }
    

    pub fn fix(&mut self) -> i32 {
        if self.block_arr.iter().any(|&pointer| pointer.y == MAX_HEIGTH) || self.block_arr.iter().any(|&pointer|{ let mut z = pointer.clone(); z.y += 1; self.pile.contains(&z)}) {
            self.block_arr.iter().for_each(|pointer| self.pile.insert(0, pointer.clone()));
            self.position = Point { x: WIDTH as i32 / 2, y: 0};
            self.block_dir = 1;
            self.direction = Direction::Down;
            1
        } else {
            0
        }
    }




    pub fn scoring(&mut self) {
        let mut y = HEIGHT as i32 - 1;
        while y >= 0 {
            if self.is_line_complete(y) {
                self.remove_complete_line(y);
                self.score += 1000;
            } else {
                y -= 1;
            }
        }
    }

    pub fn is_line_complete(&self, y: i32) -> bool {
        (0..WIDTH as i32).all(|x| self.pile.contains(&Point { x, y }))
    }

    pub fn remove_complete_line(&mut self, y: i32) {

        self.pile.retain(|point| point.y != y);
    
        self.pile.iter_mut()
            .filter(|point| point.y < y)
            .for_each(|point| point.y += 1);
        
    }


    pub fn check_game_over(&mut self) {
        if (0..WIDTH as i32).any(|x| self.pile.contains(&Point { x, y: -1 })) {
            exit(0);
        }
    }




    pub fn generating_random_block(&mut self) {
        let blocks = [Block::L, Block::J, Block::I, Block::O, Block::S, Block::Z, Block::T];
        let mut rng = rand::thread_rng();
        
        if let Some(chosen_block) = blocks.choose(&mut rng) {
            self.block_spec = *chosen_block;
        }
    }


    pub fn matching_pos_to_block(&mut self) {
        let x = self.position.x.clone();
        let y = self.position.y.clone();

        let block = match self.block_spec {
            Block::L => vec![vec![
                Point { x: x, y: y - 1 },
                Point { x: x, y: y },
                Point { x: x, y: y + 1 },
                Point { x: x + 1, y: y + 1}
            ],vec![
                Point { x: x - 1, y: y},
                Point { x: x, y: y },
                Point { x: x + 1, y: y },
                Point { x: x + 1, y: y - 1 }
            ],vec![
                Point { x: x - 1, y: y - 1 },
                Point { x: x, y: y - 1 },
                Point { x: x, y: y },
                Point { x: x , y: y + 1}
            ],vec![
                Point { x: x - 1, y: y + 1 },
                Point { x: x - 1, y: y },
                Point { x: x, y: y },
                Point { x: x + 1, y: y}
            ]],
            Block::J => vec![vec![
                Point { x: x, y: y - 1 },
                Point { x: x, y: y },
                Point { x: x, y: y + 1 },
                Point { x: x - 1, y: y + 1}
            ],vec![
                Point { x: x - 1, y: y},
                Point { x: x, y: y },
                Point { x: x + 1, y: y },
                Point { x: x + 1, y: y + 1 }
            ],vec![
                Point { x: x + 1, y: y - 1 },
                Point { x: x, y: y - 1 },
                Point { x: x, y: y },
                Point { x: x , y: y + 1}
            ],vec![
                Point { x: x - 1, y: y - 1 },
                Point { x: x - 1, y: y },
                Point { x: x, y: y },
                Point { x: x + 1, y: y}
            ]],
            Block::I => vec![vec![
                Point { x: x - 1, y: y},
                Point { x: x, y: y },
                Point { x: x + 1, y: y },
                Point { x: x + 1, y: y }
            ],vec![
                Point { x: x, y: y + 1},
                Point { x: x, y: y + 1},
                Point { x: x, y: y },
                Point { x: x, y: y - 1 }
            ],vec![
                Point { x: x - 1, y: y},
                Point { x: x, y: y },
                Point { x: x + 1, y: y },
                Point { x: x + 1, y: y }
            ],vec![
                Point { x: x, y: y + 1},
                Point { x: x, y: y + 1},
                Point { x: x, y: y },
                Point { x: x, y: y - 1 }
            ]],
            Block::O => vec![vec![
                Point { x: x, y: y },
                Point { x: x + 1, y: y },
                Point { x: x + 1, y: y + 1 },
                Point { x: x, y: y + 1}
            ],vec![
                Point { x: x, y: y },
                Point { x: x + 1, y: y },
                Point { x: x + 1, y: y + 1 },
                Point { x: x, y: y + 1}
            ],vec![
                Point { x: x, y: y },
                Point { x: x + 1, y: y },
                Point { x: x + 1, y: y + 1 },
                Point { x: x, y: y + 1}
            ],vec![
                Point { x: x, y: y },
                Point { x: x + 1, y: y },
                Point { x: x + 1, y: y + 1 },
                Point { x: x, y: y + 1}
            ]],
            Block::S => vec![vec![
                Point { x: x - 1, y: y },
                Point { x: x, y: y },
                Point { x: x, y: y - 1 },
                Point { x: x + 1, y: y - 1}
            ],vec![
                Point { x: x - 1, y: y - 1},
                Point { x: x - 1, y: y },
                Point { x: x, y: y },
                Point { x: x, y: y + 1 }
            ],vec![
                Point { x: x - 1, y: y },
                Point { x: x, y: y },
                Point { x: x, y: y - 1 },
                Point { x: x + 1, y: y - 1}
            ],vec![
                Point { x: x - 1, y: y - 1},
                Point { x: x - 1, y: y },
                Point { x: x, y: y },
                Point { x: x, y: y + 1 }
            ]],
            Block::Z => vec![vec![
                Point { x: x - 1, y: y - 1 },
                Point { x: x, y: y - 1 },
                Point { x: x, y: y },
                Point { x: x + 1, y: y }
            ],vec![
                Point { x: x + 1, y: y - 1},
                Point { x: x + 1, y: y },
                Point { x: x, y: y },
                Point { x: x, y: y + 1 }
            ],vec![
                Point { x: x - 1, y: y - 1 },
                Point { x: x, y: y - 1 },
                Point { x: x, y: y },
                Point { x: x + 1, y: y }
            ],vec![
                Point { x: x + 1, y: y - 1},
                Point { x: x + 1, y: y },
                Point { x: x, y: y },
                Point { x: x, y: y + 1 }
            ]],
            Block::T => vec![vec![
                Point { x: x, y: y - 1 },
                Point { x: x, y: y },
                Point { x: x - 1, y: y },
                Point { x: x + 1, y: y}
            ],vec![
                Point { x: x - 1, y: y},
                Point { x: x, y: y },
                Point { x: x, y: y - 1},
                Point { x: x, y: y + 1 }
            ],vec![
                Point { x: x - 1, y: y },
                Point { x: x, y: y },
                Point { x: x + 1, y: y },
                Point { x: x , y: y + 1}
            ],vec![
                Point { x: x, y: y - 1 },
                Point { x: x, y: y },
                Point { x: x, y: y + 1 },
                Point { x: x + 1, y: y}
            ]]
        };

        self.block_arr_list = block;
    }
}