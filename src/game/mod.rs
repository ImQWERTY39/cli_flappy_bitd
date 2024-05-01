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

        execute!(stdout, terminal::Clear(ClearType::All)).unwrap();

        draw_bird(&mut stdout, &bird);
        stdout.flush().unwrap();

        if let Err(_) = bird.update_height(window_height) {
            thread::sleep(Duration::from_secs(2));
            break;
        }

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

fn draw_bird(stdout: &mut Stdout, bird: &Bird) {
    for i in 0..BIRD_HEIGHT {
        queue!(stdout, cursor::MoveTo(bird.x, bird.y + i)).unwrap();

        for _ in 0..BIRD_WIDTH {
            queue!(stdout, PrintStyledContent(' '.on_yellow())).unwrap();
        }
    }
}
