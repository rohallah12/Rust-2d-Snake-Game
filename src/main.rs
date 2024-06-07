mod helpers;
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crossterm::terminal::enable_raw_mode;
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use helpers::*;
use rand::prelude::*;
use std::io::{self, Cursor, Stdout, Write};
use std::process;
use std::time::Duration;
use std::{thread, time};

//note Width of the world
const WORLD_WIDTH: u16 = 60;
//note Heigth of the world
const WORLD_HEIGHT: u16 = 60;

//Snake game
//1- Frame
//2- Snake
//3- Foods to eat
//4- Max size to win
//5- If catch self lose
//6- Foods position must be genrated randomly after eating another food

struct Speed {
    y_speed: i16,
    x_speed: i16,
}

struct Block {
    x: u16,
    y: u16,
}

struct Snake {
    blocks: Vec<Block>,
}

fn main() -> io::Result<()> {
    let stdout = &mut io::stdout();
    enable_raw_mode()?;

    stdout.execute(terminal::SetSize(WORLD_WIDTH, WORLD_HEIGHT))?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let x_pos: u16 = 10;
    let y_pos: u16 = 10;

    let speed = &mut Speed {
        y_speed: 0,
        x_speed: 1,
    };

    //Create the snake
    let snake = &mut Snake {
        blocks: vec![Block { x: x_pos, y: y_pos }],
    };

    let apple = &mut Block { x: 0, y: 0 };

    let mut collided = false;

    let mut running: bool = true;

    //initial position of the apple

    modify_food_position(stdout, apple)?;
    //this is the game loop
    loop {
        //Rest the terminal
        helpers::clear();

        println!("######Snake Game##########");
        println!("Your score: {}", snake.blocks.len());

        //Create the game frame
        create_frame(stdout, WORLD_WIDTH, WORLD_HEIGHT)?;

        //Render Apple
        render_apple(stdout, apple)?;

        //Render snake
        render_snake(stdout, snake)?;

        //Get events
        running = handle_input(snake, speed);

        //Move snake
        move_snake(snake, speed);

        //Check if collided with the apple
        collided = check_ate_apple(snake, apple);

        if collided == true {
            modify_food_position(stdout, apple)?;
            grow_snake(snake, speed);
            render_snake(stdout, snake)?;
            collided = false;
        }

        running = check_collision(snake);

        thread::sleep(time::Duration::from_millis(90));

        if running == false {
            break;
        }
    }

    stdout.flush()?;

    Ok(())
}

fn check_ate_apple(snake: &mut Snake, apple: &mut Block) -> bool {
    let head_x = snake.blocks.get(0).unwrap().x;
    let head_y = snake.blocks.get(0).unwrap().y;

    if head_x == apple.x && head_y == apple.y {
        return true;
    }

    false
}

fn check_collision(snake: &mut Snake) -> bool {
    let head_x = snake.blocks.get(0).unwrap().x;
    let head_y = snake.blocks.get(0).unwrap().y;

    if head_x == 0 || head_x == WORLD_WIDTH - 1 {
        return false;
    }

    if head_y == 0 || head_y == WORLD_HEIGHT - 1 {
        return false;
    }

    //cehck self collision
    let len = snake.blocks.len();
    if len > 2 {
        for i in 2..len {
            let x_pos = snake.blocks.get(i).unwrap().x;
            let y_pos = snake.blocks.get(i).unwrap().y;
            if head_x == x_pos && head_y == y_pos {
                return false;
            }
        }
    }

    return true;
}

fn modify_food_position(std: &mut Stdout, apple: &mut Block) -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let random = rng.next_u64();
    apple.x = (random as u16) % (WORLD_WIDTH - 20) + 10;
    apple.y = (random as u16) % (WORLD_HEIGHT - 20) + 10;
    Ok(())
}

fn render_apple(std: &mut Stdout, apple: &mut Block) -> io::Result<()> {
    create_block(std, apple.x, apple.y, '+')?;
    Ok(())
}

fn render_snake(std: &mut Stdout, snake: &mut Snake) -> io::Result<()> {
    for block in snake.blocks.iter() {
        create_block(std, block.x, block.y, '0')?;
    }

    std.execute(cursor::MoveTo(0, 0))?;

    Ok(())
}

fn move_snake(snake: &mut Snake, speed: &mut Speed) {
    let len = snake.blocks.len();

    let mut cach_x;
    let mut cach_y;
    let mut pr_x = snake.blocks[0].x;
    let mut pr_y = snake.blocks[0].y;

    for i in 0..len {
        cach_x = pr_x;
        cach_y = pr_y;
        if i == 0 {
            if speed.x_speed > 0 {
                snake.blocks[0].x += speed.x_speed as u16;
            } else {
                snake.blocks[0].x -= (-speed.x_speed) as u16;
            }

            if speed.y_speed > 0 {
                snake.blocks[0].y += speed.y_speed as u16;
            } else {
                snake.blocks[0].y -= (-speed.y_speed) as u16;
            }
        } else {
            pr_x = snake.blocks[i].x;
            pr_y = snake.blocks[i].y;
            snake.blocks[i].x = cach_x;
            snake.blocks[i].y = cach_y;
        }
    }
}

fn handle_input(snake: &mut Snake, speed: &mut Speed) -> bool {
    if poll(Duration::from_millis(100)).unwrap() {
        let key = read().unwrap();

        while poll(Duration::from_millis(0)).unwrap() {
            let _ = read();
        }
        return adjust_speed(key, snake, speed);
    }
    return true;
}

fn grow_snake(snake: &mut Snake, speed: &mut Speed) {
    let len = snake.blocks.len();
    let blk: &Block = snake.blocks.get(len - 1).unwrap();
    let mut new_block = Block { x: blk.x, y: blk.y };

    if speed.x_speed > 0 {
        new_block.x -= speed.x_speed as u16;
    } else {
        new_block.x += (-speed.x_speed) as u16;
    }

    if speed.y_speed > 0 {
        new_block.y -= speed.y_speed as u16;
    } else {
        new_block.y += (-speed.y_speed) as u16;
    }

    snake.blocks.push(new_block);
}

fn adjust_speed(key: Event, snake: &mut Snake, speed: &mut Speed) -> bool {
    let len = snake.blocks.len();
    let head_x = snake.blocks.get(0).unwrap().x;
    let head_y = snake.blocks.get(0).unwrap().y;

    match key {
        Event::Key(event) => {
            // I'm reading from keyboard into event
            match event.code {
                KeyCode::Char('w') | KeyCode::Up => {
                    if len > 1 {
                        let neck_x = snake.blocks.get(1).unwrap().x;
                        let neck_y = snake.blocks.get(1).unwrap().y;
                        if neck_y < head_y {
                            if neck_x != head_x {
                                speed.x_speed = 0;
                                speed.y_speed = -1;
                            }
                        } else {
                            speed.x_speed = 0;
                            speed.y_speed = -1;
                        }
                    } else {
                        speed.x_speed = 0;
                        speed.y_speed = -1;
                    }
                }
                KeyCode::Char('s') | KeyCode::Down => {
                    if len > 1 {
                        let neck_x = snake.blocks.get(1).unwrap().x;
                        let neck_y = snake.blocks.get(1).unwrap().y;
                        if neck_y > head_y {
                            if neck_x != head_x {
                                speed.x_speed = 0;
                                speed.y_speed = 1;
                            }
                        } else {
                            speed.x_speed = 0;
                            speed.y_speed = 1;
                        }
                    } else {
                        speed.x_speed = 0;
                        speed.y_speed = 1;
                    }
                }
                KeyCode::Char('a') | KeyCode::Left => {
                    if len > 1 {
                        let neck_x = snake.blocks.get(1).unwrap().x;
                        let neck_y = snake.blocks.get(1).unwrap().y;
                        if neck_x < head_x {
                            if neck_y != head_y {
                                speed.x_speed = -1;
                                speed.y_speed = 0;
                            }
                        } else {
                            speed.x_speed = -1;
                            speed.y_speed = 0;
                        }
                    } else {
                        speed.x_speed = -1;
                        speed.y_speed = 0;
                    }
                }
                KeyCode::Char('d') | KeyCode::Right => {
                    if len > 1 {
                        let neck_x = snake.blocks.get(1).unwrap().x;
                        let neck_y = snake.blocks.get(1).unwrap().y;
                        if neck_x > head_x {
                            if neck_y != head_y {
                                speed.x_speed = 1;
                                speed.y_speed = 0;
                            }
                        } else {
                            speed.x_speed = 1;
                            speed.y_speed = 0;
                        }
                    } else {
                        speed.x_speed = 1;
                        speed.y_speed = 0;
                    }
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    helpers::clear();
                    process::exit(1)
                }
                _ => {}
            }
        }
        _ => {}
    }
    return true;
}
