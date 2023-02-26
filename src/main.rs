use std::{
    fs::File,
    io::{self, Read, Write},
    thread::sleep,
    time::{Duration, SystemTime},
};

use sdl2::{
    event::Event,
    image::{self, InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use rand::Rng;

extern crate rand;
extern crate sdl2;

#[derive(Clone, Copy)]
enum TextureColor {
    Yellow,
    Red,
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor,
    size: u32,
) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
        canvas
            .with_texture_canvas(&mut square_texture, |texture| {
                match color {
                    TextureColor::Yellow => texture.set_draw_color(Color::YELLOW),
                    TextureColor::Red => texture.set_draw_color(Color::RED),
                }
                texture.clear();
            })
            .expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}

const TEXTURE_SIZE: u32 = 32;

fn line_to_slice(line: &str) -> Vec<u32> {
    line.split(" ")
        .filter_map(|n| n.parse::<u32>().ok())
        .collect()
}

#[test]
fn test_line_to_slice() {
    let v = line_to_slice("10 20 30");

    assert_eq!(v[0], 10);

    let a = ["1", "two", "NaN", "four", "5"];

    let mut iter = a.iter().filter_map(|s| s.parse().ok());

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);

    let a = ["1", "two", "NaN", "four", "5"];
    let mut iter = a
        .iter()
        .map(|s| s.parse())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap());
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);
}

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())
}

fn read_from_file(file_name: &str) -> io::Result<String> {
    let mut content = String::new();
    File::open(file_name)?.read_to_string(&mut content)?;
    Ok(content)
}

fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file("scores.txt") {
        let mut lines = content
            .splitn(2, "\n")
            .map(|line| line_to_slice(line))
            .collect::<Vec<_>>();

        if lines.len() == 2 {
            let (number_lines, highscores) = (lines.pop().unwrap(), lines.pop().unwrap());

            Some((highscores, number_lines))
        } else {
            None
        }
    } else {
        None
    }
}

fn slice_to_string(slice: &[u32]) -> String {
    slice
        .iter()
        .map(|highscore| highscore.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn save_highscores_and_lines(highscores: &[u32], number_of_lines: &[u32]) -> bool {
    let s_highscores = slice_to_string(highscores);
    let s_number_of_lines = slice_to_string(number_of_lines);

    write_into_file(
        &format!("{}\n{}\n", s_highscores, s_number_of_lines),
        "scores.txt",
    )
    .is_ok()
}

type Piece = Vec<Vec<u8>>;
type States = Vec<Piece>;

struct Tetrimino {
    states: States,
    x: isize,
    y: usize,
    current_state: u8,
}

trait TetriminoGenerator {
    fn new() -> Tetrimino;
}

struct TetriminoI;
impl TetriminoGenerator for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![1, 1, 1, 1],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoJ;
impl TetriminoGenerator for TetriminoJ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![2, 2, 2, 0],
                    vec![2, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 2, 0, 0],
                    vec![0, 2, 0, 0],
                    vec![0, 2, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 2, 0],
                    vec![2, 2, 2, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 0, 0, 0],
                    vec![2, 0, 0, 0],
                    vec![2, 2, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoL;
impl TetriminoGenerator for TetriminoL {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![3, 3, 3, 0],
                    vec![0, 0, 3, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 3, 0, 0],
                    vec![0, 3, 0, 0],
                    vec![3, 3, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 0, 0, 0],
                    vec![3, 3, 3, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 3, 0, 0],
                    vec![3, 0, 0, 0],
                    vec![3, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoO;
impl TetriminoGenerator for TetriminoO {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![
                vec![4, 4, 0, 0],
                vec![4, 4, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]],
            x: 5,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoS;
impl TetriminoGenerator for TetriminoS {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![0, 5, 5, 0],
                    vec![5, 5, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 5, 0, 0],
                    vec![0, 5, 5, 0],
                    vec![0, 0, 5, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoZ;
impl TetriminoGenerator for TetriminoZ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![6, 6, 0, 0],
                    vec![0, 6, 6, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 6, 0],
                    vec![0, 6, 6, 0],
                    vec![0, 6, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoT;
impl TetriminoGenerator for TetriminoT {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![7, 7, 7, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 0, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 7, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![0, 7, 7, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

impl Tetrimino {
    fn rotate(&mut self) {
        self.current_state += 1;
        if self.current_state as usize >= self.states.len() {
            self.current_state = 0;
        }
    }
}

fn create_new_tetrimino() -> Tetrimino {
    static mut PREV: u8 = 7;
    let mut which = rand::thread_rng().gen_range(0..7);

    if unsafe { PREV } == which {
        which = rand::thread_rng().gen_range(0..7);
    }

    unsafe {
        PREV = which;
    }

    match which {
        0 => TetriminoI::new(),
        1 => TetriminoJ::new(),
        2 => TetriminoL::new(),
        3 => TetriminoO::new(),
        4 => TetriminoS::new(),
        5 => TetriminoZ::new(),
        6 => TetriminoT::new(),
        _ => unreachable!(),
    }
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL Initialization Failed.");

    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get SDL Video Subsystem");

    let window = video_subsystem
        .window("Rust Tetris Game", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Failed to convert window into canvas.");

    image::init(InitFlag::PNG | InitFlag::JPG).expect("Couldn't initialize imagem context");

    let texture_creator = canvas.texture_creator();

    let image_texture = texture_creator
        .load_texture("assets/rust-logo.jpg")
        .expect("Couldn't load imagem");

    let mut square_texture = texture_creator
        .create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
        .expect("Failed to create texture");

    canvas
        .with_texture_canvas(&mut square_texture, |texture| {
            texture.set_draw_color(Color::YELLOW);
            texture.clear();
        })
        .expect("Failed to config texture color");

    let red_square = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Red,
        TEXTURE_SIZE,
    )
    .expect("Failed to create a texture");
    let yellow_square = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Yellow,
        TEXTURE_SIZE,
    )
    .expect("Failed to create a texture");

    let timer = SystemTime::now();

    let mut event_bump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event bump");

    let highscores = [10, 20, 30];
    let number_of_lines = [2, 3, 4];
    save_highscores_and_lines(&highscores, &number_of_lines);

    'running: loop {
        for event in event_bump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let display_red = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_) => true,
        };

        let square_texture = if display_red {
            &red_square
        } else {
            &yellow_square
        };

        canvas
            .copy(
                square_texture,
                None,
                Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE),
            )
            .expect("Couldn't copy texture into window");

        canvas
            .copy(&image_texture, None, Rect::new(40, 40, 400, 400))
            .expect("Couldn't copy texture into window");

        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
