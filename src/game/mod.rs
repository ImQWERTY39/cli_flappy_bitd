use std::{
    io::{Stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{PrintStyledContent, Stylize},
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

    let mut bird = Bird {
        x: BIRD_WIDTH,
        y: (window_height - BIRD_HEIGHT) / 2 - 1,
        y_vel: 0,
    };
    let mut pipes: Vec<Pipe> = Vec::new();

    loop {
        if event::poll(WAIT_TIME).unwrap() {
            match event::read().unwrap() {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Esc => break,
                    KeyCode::Char(i) => {
                        if i == ' ' {
                            bird.y_vel += JUMP_STRENGTH;
                        }
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

        draw_pipes(&mut stdout, &pipes, window_height);
        draw_bird(&mut stdout, &bird);
        stdout.flush().unwrap();

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

fn draw_pipes(stdout: &mut Stdout, pipes: &[Pipe], window_height: u16) {
    pipes
        .iter()
        .for_each(|x| draw_pipe(stdout, x, window_height));
}

fn draw_pipe(stdout: &mut Stdout, pipe: &Pipe, window_height: u16) {
    for i in 0..window_height {
        // if ingap { continue }

        queue!(stdout, cursor::MoveTo(pipe.x, i)).unwrap();

        for _ in 0..PIPE_WIDTH {
            queue!(stdout, PrintStyledContent(' '.on_green())).unwrap();
        }
    }
}

fn draw_bird(stdout: &mut Stdout, bird: &Bird) {
    for i in 0..BIRD_HEIGHT {
        queue!(stdout, cursor::MoveTo(bird.x, bird.y + i)).unwrap();

        for _ in 0..BIRD_WIDTH {
            queue!(stdout, PrintStyledContent(' '.on_yellow())).unwrap();
        }
    }
}
