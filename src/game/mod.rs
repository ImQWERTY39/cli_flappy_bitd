use std::{
    io::{Stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{PrintStyledContent, StyledContent, Stylize},
    terminal::{self, ClearType},
};
use rand::Rng;

use crate::{bird::*, pipe::*};

const WAIT_TIME: Duration = Duration::from_millis(25);

pub fn main_loop() {
    let mut stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode().unwrap();
    execute!(
        stdout,
        cursor::Hide,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();

    let (window_width, window_height) = crossterm::terminal::size().unwrap();
    let mut rand = rand::thread_rng();

    let mut screen = vec![vec![' '.reset(); window_width as usize]; window_height as usize];

    let mut bird = Bird {
        x: BIRD_WIDTH,
        y: (window_height - BIRD_HEIGHT) / 2 - 1,
        y_vel: 0,
    };
    let mut pipes: Vec<Pipe> = Vec::new();

    loop {
        for i in screen.iter_mut() {
            for j in i {
                *j = ' '.reset();
            }
        }

        if event::poll(WAIT_TIME).unwrap() {
            match event::read().unwrap() {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Esc => break,
                    KeyCode::Char(' ') => {
                        bird.y_vel += JUMP_STRENGTH;
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        if rand.gen::<u8>() == 1 {
            pipes.push(Pipe {
                x: window_width,
                gap_y: rand.gen_range(2..(window_width - PIPE_GAP_HEIGHT - 2)),
            })
        }

        execute!(stdout, terminal::Clear(ClearType::All)).unwrap();

        draw_pipes(&mut screen, &pipes, window_height);
        draw_bird(&mut screen, &bird);
        draw_screen(&mut stdout, &screen);

        if let Err(_) = bird.update_height(window_height) {
            thread::sleep(Duration::from_secs(2));
            break;
        }

        pipes = update_pipes(pipes);

        thread::sleep(WAIT_TIME);
    }

    execute!(
        stdout,
        cursor::Show,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
}

fn update_pipes(pipes: Vec<Pipe>) -> Vec<Pipe> {
    pipes
        .into_iter()
        .map(|mut x| {
            x.x = x.x.checked_sub(PIPE_VEL).unwrap_or_default();
            x
        })
        .filter(|x| x.x > 0)
        .collect()
}

fn draw_pipes(screen: &mut Vec<Vec<StyledContent<char>>>, pipes: &[Pipe], window_height: u16) {
    pipes
        .iter()
        .for_each(|x| draw_pipe(screen, x, window_height));
}

fn draw_pipe(screen: &mut Vec<Vec<StyledContent<char>>>, pipe: &Pipe, window_height: u16) {
    for i in 0..window_height {
        if i >= pipe.gap_y && pipe.gap_y - i < PIPE_GAP_HEIGHT {
            continue;
        }

        for j in 0..PIPE_WIDTH {
            screen[i as usize][(pipe.x + j) as usize] = ' '.on_green();
        }
    }
}

fn draw_bird(screen: &mut Vec<Vec<StyledContent<char>>>, bird: &Bird) {
    for i in 0..BIRD_HEIGHT {
        for j in 0..BIRD_WIDTH {
            screen[(bird.y + i) as usize][(bird.x + j) as usize] = ' '.on_yellow();
        }
    }
}

fn draw_screen(stdout: &mut Stdout, screen: &Vec<Vec<StyledContent<char>>>) {
    for (r, row) in screen.iter().enumerate() {
        queue!(stdout, cursor::MoveTo(0, r as u16)).unwrap();

        for chr in row {
            queue!(stdout, PrintStyledContent(chr.clone())).unwrap();
        }
    }

    stdout.flush().unwrap();
}
