// for displaying window
use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
// for drawing graphics to window
use raqote::{
    DrawOptions, DrawTarget, Image, LineCap, LineJoin, PathBuilder, Point, SolidSource, Source,
    StrokeStyle, Transform,
};
// for computing transformations
use euclid::Angle;
// for fonts
use font_kit::loaders::default::Font;
// for etc.
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

macro_rules! is_open {
    ($window:expr) => {
        $window.is_open()
    };
}

// here's how stdg can be used by a client
//
// - client prints start command, first before anything else they print
// - client prints whatever they want, after the start command
// - if client prints a line starting with documented command, that line is interpreted as a command to stdg
// - otherwise, the line gets printed to terminal/stdout
// - client prints present in an infinite loop to infinitely present (draw) the graphics
// - client prints get in an infinte loop to get an event
// - then, info regarding the output is printed to the client's stdin
// - then, client can read for that info
//
// there are only 2 big caveats of client programs
// 1. reading from input from your user is not possible if and only if you are passing your process/command as an argument to stdg (but if you use your_program | stdg, you are fine)
// 2. printing to output is not possible if and only if you want your output to start with one of stdg's reserved commands (so in most cases, printing behaves as normal)

fn main() {
    // one thing to note-
    // we only try to make things work smoothly if there are no panics on the
    // Rust side and no exceptions on the Python/other language side
    //
    // so if there is a panic/exception, the window might hang, infinite loops
    // may run
    // while we will try to make things still work smoothly when panics/
    // exceptions happen, we aren't promising anything
    //
    // notably though, if the user closes a window, we properly close the
    // Python/other language process that was running
    // this is because the user closing a window is, of course, normal
    // expected behavior and it makes sense to just close the process
    // of course, if we are piping from some other process, we can't close
    // that process, and we will inevitably end up with a broken process
    //
    // ok, on to the code...

    // accept command line arguments
    let args: Vec<String> = env::args().collect();

    // if we have an argument passed in, this is the process to launch
    // so, we launch the given process
    // then, get handles to stdin and stdout to and from the
    //
    // otherwise, we just get a handle to stdin to read from
    let mut process = if args.len() >= 2 {
        Some(
            Command::new(&args[1])
                .args(&args[2..])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute process"),
        )
    } else {
        None
    };
    let (mut process_in, mut process_out_reader, mut stdin_reader) = if args.len() >= 2 {
        (
            Some(
                process
                    .as_mut()
                    .unwrap()
                    .stdin
                    .take()
                    .expect("failed to get stdin to process"),
            ),
            Some(BufReader::new(
                process
                    .as_mut()
                    .unwrap()
                    .stdout
                    .take()
                    .expect("failed to get stdout from process"),
            )),
            Some(std::io::stdin()),
        )
    } else {
        (None, None, Some(std::io::stdin()))
    };

    // keep track of what we have read so far
    let mut reading = String::new();

    // get window command
    if process.is_some() {
        process_out_reader
            .as_mut()
            .unwrap()
            .read_line(&mut reading)
            .expect("expected start command");
    } else {
        stdin_reader
            .as_mut()
            .unwrap()
            .read_line(&mut reading)
            .expect("expected start command");
    }
    let window_info = reading
        .split_whitespace()
        .map(|token| token.to_string())
        .collect::<Vec<String>>();
    reading.clear();
    assert_eq!(
        window_info.get(0).expect("expected start command").as_str(),
        "start"
    );
    let window_width = window_info
        .get(1)
        .expect("expected width of window")
        .parse::<usize>()
        .expect("expected integer (usize) width of window");
    let window_height = window_info
        .get(2)
        .expect("expected height of window")
        .parse::<usize>()
        .expect("expected integer (usize) height of window");
    let window_title = window_info
        .get(3..)
        .expect("expected title of window")
        .join(" ");

    // create the window
    // initialize the window as draw target
    // intialize window width and height
    let mut window = Window::new(
        window_title.as_str(),
        window_width,
        window_height,
        WindowOptions {
            resize: false,
            scale: Scale::X1,
            ..WindowOptions::default()
        },
    )
    .expect("failed to create window");
    let mut draw_target = DrawTarget::new(window_width as i32, window_height as i32);

    // keep track of some stuff
    let mut fill: Option<(u8, u8, u8, u8)> = None;
    let mut stroke = Some((0, 0, 0, 255));
    let mut stroke_weight = 1.0;
    let mut stroke_cap = LineCap::Round;
    let mut stroke_join = LineJoin::Miter;
    let mut transformations = vec![draw_target.get_transform().clone()];
    let mut text_font = None;
    let mut text_size = None;
    let mut images: HashMap<String, _> = HashMap::new();

    // draw forever
    while is_open!(window) {
        // read a line from process
        if process.is_some() {
            process_out_reader
                .as_mut()
                .unwrap()
                .read_line(&mut reading)
                .expect("expected command to draw graphics");
        } else {
            stdin_reader
                .as_mut()
                .unwrap()
                .read_line(&mut reading)
                .expect("expected command to draw graphics");
        }
        let command = reading
            .split_whitespace()
            .map(|token| token.to_string())
            .collect::<Vec<String>>();

        // handle the command from line
        if command.len() >= 3 && command.len() % 2 == 1 && command[0].as_str() == "poly" {
            let mut pb = PathBuilder::new();

            let mut is_first = true;
            for point in command[1..].chunks(2) {
                if is_first {
                    is_first = false;
                    pb.move_to(
                        point[0]
                            .parse::<f32>()
                            .expect("expected x position (f32) of vertex of polygon"),
                        point[1]
                            .parse::<f32>()
                            .expect("expected y position (f32) of vertex of polygon"),
                    );
                } else {
                    pb.line_to(
                        point[0]
                            .parse::<f32>()
                            .expect("expected x position (f32) of vertex of polygon"),
                        point[1]
                            .parse::<f32>()
                            .expect("expected y position (f32) of vertex of polygon"),
                    );
                }
            }
            let path = pb.finish();

            if let Some(fill_color) = &fill {
                let source = Source::Solid(SolidSource {
                    r: fill_color.0,
                    g: fill_color.1,
                    b: fill_color.2,
                    a: 255,
                });
                draw_target.fill(
                    &path,
                    &source,
                    &DrawOptions {
                        alpha: fill_color.3 as f32 / 255.0,
                        ..DrawOptions::default()
                    },
                );
            }
            if let Some(stroke_color) = &stroke {
                let source = Source::Solid(SolidSource {
                    r: stroke_color.0,
                    g: stroke_color.1,
                    b: stroke_color.2,
                    a: stroke_color.3,
                });
                draw_target.stroke(
                    &path,
                    &source,
                    &StrokeStyle {
                        width: stroke_weight,
                        cap: stroke_cap,
                        join: stroke_join,
                        ..StrokeStyle::default()
                    },
                    &DrawOptions::default(),
                );
            }
        } else {
            match command.len() {
                1 => match command[0].as_str() {
                    "present" => {
                        // draw everything
                        window.update_with_buffer(draw_target.get_data()).unwrap();
                    }
                    "nofill" => {
                        fill = None;
                    }
                    "nostroke" => {
                        stroke = None;
                    }
                    "push" => {
                        if transformations.len() == 0 {
                            panic!("invalid usage of push and pop");
                        }
                        transformations.push(transformations.last().unwrap().clone());
                    }
                    "pop" => {
                        if transformations.len() == 0 {
                            panic!("no transformation to pop");
                        }
                        transformations.pop();
                        if transformations.len() == 0 {
                            panic!("pop was not expected");
                        }
                        draw_target.set_transform(transformations.last().unwrap());
                    }
                    _ => {
                        print!("{}", reading);
                    }
                },
                2 => match command[0].as_str() {
                    "present" => match command[1].as_str() {
                        "forever" => {
                            while is_open!(window) {
                                // draw everything
                                window.update_with_buffer(draw_target.get_data()).unwrap();
                            }
                            break;
                        }
                        _ => {
                            panic!("unsupported usage of present command");
                        }
                    },
                    "get" => match command[1].as_str() {
                        "mousex" => {
                            window.update();
                            writeln!(process_in.as_mut().expect("process must be passed as argument to stdg for mouse x position to be printed to it"), "{}", window.get_mouse_pos(MouseMode::Pass).expect("failed to get x position of mouse").0);
                        }
                        "mousey" => {
                            window.update();
                            writeln!(process_in.as_mut().expect("process must be passed as argument to stdg for mouse y position to be printed to it"), "{}", window.get_mouse_pos(MouseMode::Pass).expect("failed to get y position of mouse").1);
                        }
                        "keys" => {
                            window.update();
                            let mut output = String::new();
                            for key in window.get_keys().expect("failed to get keys pressed") {
                                if output.len() > 0 {
                                    output += " ";
                                }
                                output += match key {
                                    Key::Key0 => "0",
                                    Key::Key1 => "1",
                                    Key::Key2 => "2",
                                    Key::Key3 => "3",
                                    Key::Key4 => "4",
                                    Key::Key5 => "5",
                                    Key::Key6 => "6",
                                    Key::Key7 => "7",
                                    Key::Key8 => "8",
                                    Key::Key9 => "9",
                                    Key::A => "a",
                                    Key::B => "b",
                                    Key::C => "c",
                                    Key::D => "d",
                                    Key::E => "e",
                                    Key::F => "f",
                                    Key::G => "g",
                                    Key::H => "h",
                                    Key::I => "i",
                                    Key::J => "j",
                                    Key::K => "k",
                                    Key::L => "l",
                                    Key::M => "m",
                                    Key::N => "n",
                                    Key::O => "o",
                                    Key::P => "p",
                                    Key::Q => "q",
                                    Key::R => "r",
                                    Key::S => "s",
                                    Key::T => "t",
                                    Key::U => "u",
                                    Key::V => "v",
                                    Key::W => "w",
                                    Key::X => "x",
                                    Key::Y => "y",
                                    Key::Z => "z",
                                    Key::Up => "up",
                                    Key::Down => "down",
                                    Key::Left => "left",
                                    Key::Right => "right",
                                    Key::Space => "space",
                                    Key::Tab => "tab",
                                    Key::Enter => "enter",
                                    Key::LeftShift => "leftshift",
                                    Key::RightShift => "rightshift",
                                    Key::Escape => "escape",
                                    Key::Backspace => "backspace",
                                    Key::Delete => "delete",
                                    _ => "unknown",
                                }
                            }
                            writeln!(process_in.as_mut().expect("process must be passed as argument to stdg for keys pressed to be printed to it"), "{}", output);
                        }
                        _ => {
                            panic!("unsupported usage of get command");
                        }
                    },
                    "save" => {
                        draw_target
                            .write_png(command[1].clone())
                            .expect("failed to save");
                    }
                    "strokeweight" => {
                        stroke_weight = command[1]
                            .parse::<f32>()
                            .expect("expected stroke weight (f32)")
                    }
                    "strokecap" => {
                        stroke_cap = match command[1].as_str() {
                            "square" => LineCap::Square,
                            "project" => LineCap::Butt,
                            "round" => LineCap::Round,
                            _ => panic!("expected either square, project, or round for stroke cap"),
                        };
                    }
                    "strokejoin" => {
                        stroke_join = match command[1].as_str() {
                            "miter" => LineJoin::Miter,
                            "bevel" => LineJoin::Bevel,
                            "round" => LineJoin::Round,
                            _ => panic!("expected either miter, bevel, or round for stroke join"),
                        };
                    }
                    "rotate" => {
                        if transformations.len() == 0 {
                            panic!("invalid usage of push and pop");
                        }
                        *transformations.last_mut().unwrap() = transformations
                            .last()
                            .unwrap()
                            .post_transform(&Transform::create_rotation(Angle::degrees(
                                command[1].parse::<f32>().expect(
                                    "expected rotation value in degrees (f32) for rotation",
                                ),
                            )));
                        draw_target.set_transform(&transformations.last().unwrap());
                    }
                    "textfont" => {
                        text_font = Some(
                            Font::from_path(command[1].clone(), 0).expect("failed to load font"),
                        );
                    }
                    "textsize" => {
                        text_size = Some(
                            command[1]
                                .parse::<f32>()
                                .expect("expected size of text (f32)"),
                        );
                    }
                    _ => {
                        print!("{}", reading);
                    }
                },
                3 => match command[0].as_str() {
                    "get" => match command[1].as_str() {
                        "mouseispressed" => match command[2].as_str() {
                            "left" => {
                                window.update();
                                writeln!(process_in.as_mut().expect("process must be passed as argument to stdg for boolean answer to \"is mouse pressed?\" to be printed to it"), "{}", window.get_mouse_down(MouseButton::Left));
                            }
                            "center" => {
                                window.update();
                                writeln!(process_in.as_mut().expect("process must be passed as argument to stdg for boolean answer to \"is mouse pressed?\" to be printed to it"), "{}", window.get_mouse_down(MouseButton::Middle));
                            }
                            "right" => {
                                window.update();
                                writeln!(process_in.as_mut().expect("process must be passed as argument to stdg for boolean answer to \"is mouse pressed?\" to be printed to it"), "{}", window.get_mouse_down(MouseButton::Right));
                            }
                            _ => panic!(
                                "expected either left, center, or right mouse button ask if pressed"
                            ),
                        },
                        "keyispressed" => {
                            window.update();
                            writeln!(process_in.as_mut().expect("process must be passed as argument to stdg for boolean answer to \"is key pressed?\" to be printed to it"), "{}", window.is_key_down(match command[2].as_str() {
                                "0" => Key::Key0,
                                "1" => Key::Key1,
                                "2" => Key::Key2,
                                "3" => Key::Key3,
                                "4" => Key::Key4,
                                "5" => Key::Key5,
                                "6" => Key::Key6,
                                "7" => Key::Key7,
                                "8" => Key::Key8,
                                "9" => Key::Key9,
                                "a" => Key::A,
                                "b" => Key::B,
                                "c" => Key::C,
                                "d" => Key::D,
                                "e" => Key::E,
                                "f" => Key::F,
                                "g" => Key::G,
                                "h" => Key::H,
                                "i" => Key::I,
                                "j" => Key::J,
                                "k" => Key::K,
                                "l" => Key::L,
                                "m" => Key::M,
                                "n" => Key::N,
                                "o" => Key::O,
                                "p" => Key::P,
                                "q" => Key::Q,
                                "r" => Key::R,
                                "s" => Key::S,
                                "t" => Key::T,
                                "u" => Key::U,
                                "v" => Key::V,
                                "w" => Key::W,
                                "x" => Key::X,
                                "y" => Key::Y,
                                "z" => Key::Z,
                                "up" => Key::Up,
                                "down" => Key::Down,
                                "left" => Key::Left,
                                "right" => Key::Right,
                                "space" => Key::Space,
                                "tab" => Key::Tab,
                                "enter" => Key::Enter,
                                "leftshift" => Key::LeftShift,
                                "rightshift" => Key::RightShift,
                                "escape" => Key::Escape,
                                "backspace" => Key::Backspace,
                                "delete" => Key::Delete,
                                _ => panic!("unsupported key used")
                            }));
                        }
                        _ => {
                            panic!("unsupported usage of get command");
                        }
                    },
                    "translate" => {
                        if transformations.len() == 0 {
                            panic!("invalid usage of push and pop");
                        }
                        *transformations.last_mut().unwrap() = transformations
                            .last()
                            .unwrap()
                            .post_transform(&Transform::create_translation(
                                command[1]
                                    .parse::<f32>()
                                    .expect("expected x value (f32) for translation"),
                                command[2]
                                    .parse::<f32>()
                                    .expect("expected y value (f32) for translation"),
                            ));
                        draw_target.set_transform(&transformations.last().unwrap());
                    }
                    "scale" => {
                        if transformations.len() == 0 {
                            panic!("invalid usage of push and pop");
                        }
                        *transformations.last_mut().unwrap() = transformations
                            .last()
                            .unwrap()
                            .post_transform(&Transform::create_scale(
                                command[1]
                                    .parse::<f32>()
                                    .expect("expected x value (f32) for scaling"),
                                command[2]
                                    .parse::<f32>()
                                    .expect("expected y value (f32) for scaling"),
                            ));
                        draw_target.set_transform(&transformations.last().unwrap());
                    }
                    "text" => {
                        if let Some(fill_color) = fill {
                            let source = Source::Solid(SolidSource {
                                r: fill_color.0,
                                g: fill_color.1,
                                b: fill_color.2,
                                a: 255,
                            });

                            let mut text_to_draw = String::new();
                            if process.is_some() {
                                process_out_reader
                                    .as_mut()
                                    .unwrap()
                                    .read_line(&mut text_to_draw)
                                    .expect("expected command to draw graphics");
                            } else {
                                stdin_reader
                                    .as_mut()
                                    .unwrap()
                                    .read_line(&mut text_to_draw)
                                    .expect("expected command to draw graphics");
                            }
                            draw_target.draw_text(
                                &text_font
                                    .clone()
                                    .expect("text font must be given before text is drawn"),
                                text_size.expect("text size must be given before text is drawn"),
                                text_to_draw
                                    .split_whitespace()
                                    .collect::<Vec<&str>>()
                                    .join(" ")
                                    .as_str(),
                                Point::new(
                                    command[1]
                                        .parse::<f32>()
                                        .expect("expected x position of text (f32)"),
                                    command[2]
                                        .parse::<f32>()
                                        .expect("expected y position of text (f32)"),
                                ),
                                &source,
                                &DrawOptions {
                                    alpha: fill_color.3 as f32 / 255.0,
                                    ..DrawOptions::default()
                                },
                            );
                        }
                    }
                    _ => {
                        print!("{}", reading);
                    }
                },
                4 => match command[0].as_str() {
                    "open" => {
                        assert_eq!(command[2].as_str(), "as");
                    	let decoder = png::Decoder::new(
                            File::open(command[1].as_str()).expect("failed to open file"),
                        );
                        let (info, mut reader) = decoder.read_info().expect("failed to open file");
                        let width = info.width;
                        let height = info.height;
                        let mut buf = vec![0; info.buffer_size()];
                        reader.next_frame(&mut buf).unwrap();
                        let buf_usize = buf
                            .clone()
                            .iter()
                            .map(|byte| *byte as u32)
                            .collect::<Vec<u32>>();
                    	images.insert(command[3].clone(), (width, height, buf_usize));
                    }
                    "image" => {
                    	draw_target.draw_image_at(
                    		command[2]
                                .parse::<f32>()
                                .expect("expected x position (f32) of image"),
                            command[3]
                                .parse::<f32>()
                                .expect("expected y position (f32) of image"),
                            &Image {
                            	width: images.get(&command[1]).expect("image not opened yet").0 as i32,
                            	height: images.get(&command[1]).expect("image not opened yet").1 as i32,
                            	data: &images.get(&command[1]).expect("image not opened yet").2,
                            },
                            &DrawOptions::default()
                    	);
                    }
                    "background" => {
                        draw_target.clear(SolidSource {
                            r: command[1]
                                .parse::<u8>()
                                .expect("expected red value (u8) of color"),
                            g: command[2]
                                .parse::<u8>()
                                .expect("expected green value (u8) of color"),
                            b: command[3]
                                .parse::<u8>()
                                .expect("expected blue value (u8) of color"),
                            a: 0,
                        });
                    }
                    "fill" => {
                        fill = Some((
                            command[1]
                                .parse::<u8>()
                                .expect("expected red value (u8) of color"),
                            command[2]
                                .parse::<u8>()
                                .expect("expected green value (u8) of color"),
                            command[3]
                                .parse::<u8>()
                                .expect("expected blue value (u8) of color"),
                            255,
                        ));
                    }
                    "stroke" => {
                        stroke = Some((
                            command[1]
                                .parse::<u8>()
                                .expect("expected red value (u8) of color"),
                            command[2]
                                .parse::<u8>()
                                .expect("expected green value (u8) of color"),
                            command[3]
                                .parse::<u8>()
                                .expect("expected blue value (u8) of color"),
                            255,
                        ));
                    }
                    "circle" => {
                        let x = command[1]
                            .parse::<f32>()
                            .expect("expected x position (f32) of circle");
                        let y = command[2]
                            .parse::<f32>()
                            .expect("expected y position (f32) of circle");
                        let r = command[3]
                            .parse::<f32>()
                            .expect("expected radius (f32) of circle");

                        let mut pb = PathBuilder::new();
                        pb.move_to(x + r, y);
                        pb.arc(x, y, r, 0.0, 2.0 * std::f32::consts::PI);
                        let path = pb.finish();

                        if let Some(stroke_color) = &stroke {
                            let source = Source::Solid(SolidSource {
                                r: stroke_color.0,
                                g: stroke_color.1,
                                b: stroke_color.2,
                                a: stroke_color.3,
                            });
                            draw_target.stroke(
                                &path,
                                &source,
                                &StrokeStyle {
                                    width: stroke_weight,
                                    cap: stroke_cap,
                                    join: stroke_join,
                                    ..StrokeStyle::default()
                                },
                                &DrawOptions::default(),
                            );
                        }
                    }
                    _ => {
                        print!("{}", reading);
                    }
                },
                5 => match command[0].as_str() {
                    "ellipse" => {
                        // get dimensions
                        let x = command[1]
                            .parse::<f32>()
                            .expect("expected x position (f32) of ellipse");
                        let y = command[2]
                            .parse::<f32>()
                            .expect("expected y position (f32) of ellipse");
                        let w = command[3]
                            .parse::<f32>()
                            .expect("expected width (f32) of ellipse");
                        let h = command[4]
                            .parse::<f32>()
                            .expect("expected height (f32) of ellipse");

                        // push scaling transform
                        transformations.push(transformations.last().unwrap().clone());

                        // scale
                        if transformations.len() == 0 {
                            panic!("invalid usage of push and pop");
                        }
                        *transformations.last_mut().unwrap() = transformations
                            .last()
                            .unwrap()
                            .post_transform(&Transform::create_scale(1.0, h / w));
                        draw_target.set_transform(&transformations.last().unwrap());

                        // draw the ellipse
                        let mut pb = PathBuilder::new();
                        pb.move_to(x + w / 2.0, y);
                        pb.arc(x, y, w / 2.0, 0.0, 2.0 * std::f32::consts::PI);
                        let path = pb.finish();

                        if let Some(stroke_color) = &stroke {
                            let source = Source::Solid(SolidSource {
                                r: stroke_color.0,
                                g: stroke_color.1,
                                b: stroke_color.2,
                                a: stroke_color.3,
                            });
                            draw_target.stroke(
                                &path,
                                &source,
                                &StrokeStyle {
                                    width: stroke_weight,
                                    cap: stroke_cap,
                                    join: stroke_join,
                                    ..StrokeStyle::default()
                                },
                                &DrawOptions::default(),
                            );
                        }

                        // pop scaling tansform
                        transformations.pop();
                        draw_target.set_transform(transformations.last().unwrap());
                    }
                    "line" => {
                        let mut pb = PathBuilder::new();
                        pb.move_to(
                            command[1]
                                .parse::<f32>()
                                .expect("expected x position (f32) of rectangle"),
                            command[2]
                                .parse::<f32>()
                                .expect("expected y position (f32) of rectangle"),
                        );
                        pb.line_to(
                            command[3]
                                .parse::<f32>()
                                .expect("expected width (f32) of rectangle"),
                            command[4]
                                .parse::<f32>()
                                .expect("expected height (f32) of rectangle"),
                        );
                        let path = pb.finish();

                        if let Some(fill_color) = &fill {
                            let source = Source::Solid(SolidSource {
                                r: fill_color.0,
                                g: fill_color.1,
                                b: fill_color.2,
                                a: 255,
                            });
                            draw_target.fill(
                                &path,
                                &source,
                                &DrawOptions {
                                    alpha: fill_color.3 as f32 / 255.0,
                                    ..DrawOptions::default()
                                },
                            );
                        }
                        if let Some(stroke_color) = &stroke {
                            let source = Source::Solid(SolidSource {
                                r: stroke_color.0,
                                g: stroke_color.1,
                                b: stroke_color.2,
                                a: stroke_color.3,
                            });
                            draw_target.stroke(
                                &path,
                                &source,
                                &StrokeStyle {
                                    width: stroke_weight,
                                    cap: stroke_cap,
                                    join: stroke_join,
                                    ..StrokeStyle::default()
                                },
                                &DrawOptions::default(),
                            );
                        }
                    }
                    "rect" => {
                        let mut pb = PathBuilder::new();
                        pb.rect(
                            command[1]
                                .parse::<f32>()
                                .expect("expected x position (f32) of rectangle"),
                            command[2]
                                .parse::<f32>()
                                .expect("expected y position (f32) of rectangle"),
                            command[3]
                                .parse::<f32>()
                                .expect("expected width (f32) of rectangle"),
                            command[4]
                                .parse::<f32>()
                                .expect("expected height (f32) of rectangle"),
                        );
                        let path = pb.finish();

                        if let Some(fill_color) = &fill {
                            let source = Source::Solid(SolidSource {
                                r: fill_color.0,
                                g: fill_color.1,
                                b: fill_color.2,
                                a: 255,
                            });
                            draw_target.fill(
                                &path,
                                &source,
                                &DrawOptions {
                                    alpha: fill_color.3 as f32 / 255.0,
                                    ..DrawOptions::default()
                                },
                            );
                        }
                        if let Some(stroke_color) = &stroke {
                            let source = Source::Solid(SolidSource {
                                r: stroke_color.0,
                                g: stroke_color.1,
                                b: stroke_color.2,
                                a: stroke_color.3,
                            });
                            draw_target.stroke(
                                &path,
                                &source,
                                &StrokeStyle {
                                    width: stroke_weight,
                                    cap: stroke_cap,
                                    join: stroke_join,
                                    ..StrokeStyle::default()
                                },
                                &DrawOptions::default(),
                            );
                        }
                    }
                    "fill" => {
                        fill = Some((
                            command[1]
                                .parse::<u8>()
                                .expect("expected red value (u8) of color"),
                            command[2]
                                .parse::<u8>()
                                .expect("expected green value (u8) of color"),
                            command[3]
                                .parse::<u8>()
                                .expect("expected blue value (u8) of color"),
                            command[4]
                                .parse::<u8>()
                                .expect("expected alpha (transparency, u8) value of color"),
                        ));
                    }
                    "stroke" => {
                        stroke = Some((
                            command[1]
                                .parse::<u8>()
                                .expect("expected red value (u8) of color"),
                            command[2]
                                .parse::<u8>()
                                .expect("expected green value (u8) of color"),
                            command[3]
                                .parse::<u8>()
                                .expect("expected blue value (u8) of color"),
                            command[4]
                                .parse::<u8>()
                                .expect("expected alpha (transparency, u8) value of color"),
                        ));
                    }
                    _ => {
                        print!("{}", reading);
                    }
                },
                6 => match command[0].as_str() {
                	"image" => {
                    	draw_target.draw_image_with_size_at(
                    		command[4]
                                .parse::<f32>()
                                .expect("expected width (f32) of image"),
                            command[5]
                                .parse::<f32>()
                                .expect("expected height (f32) of image"),
                    		command[2]
                                .parse::<f32>()
                                .expect("expected x position (f32) of image"),
                            command[3]
                                .parse::<f32>()
                                .expect("expected y position (f32) of image"),
                            &Image {
                            	width: images.get(&command[1]).expect("image not opened yet").0 as i32,
                            	height: images.get(&command[1]).expect("image not opened yet").1 as i32,
                            	data: &images.get(&command[1]).expect("image not opened yet").2,
                            },
                            &DrawOptions::default()
                    	);
                    }
                    "arc" => {
                        let x = command[1]
                            .parse::<f32>()
                            .expect("expected x position (f32) of arc");
                        let y = command[2]
                            .parse::<f32>()
                            .expect("expected y position (f32) of arc");
                        let r = command[3]
                            .parse::<f32>()
                            .expect("expected radius (f32) of arc");
                        let start_angle = command[4]
                            .parse::<f32>()
                            .expect("expected start angle (f32) of arc");
                        let end_angle = command[5]
                            .parse::<f32>()
                            .expect("expected end angle (f32) of arc");

                        let mut pb = PathBuilder::new();
                        pb.move_to(x + r * start_angle.cos(), y + r * start_angle.sin());
                        pb.arc(
                            x,
                            y,
                            r,
                            start_angle,
                            2.0 * std::f32::consts::PI * end_angle / 360.0,
                        );
                        let path = pb.finish();

                        if let Some(fill_color) = &fill {
                            let source = Source::Solid(SolidSource {
                                r: fill_color.0,
                                g: fill_color.1,
                                b: fill_color.2,
                                a: 255,
                            });
                            draw_target.fill(
                                &path,
                                &source,
                                &DrawOptions {
                                    alpha: fill_color.3 as f32 / 255.0,
                                    ..DrawOptions::default()
                                },
                            );
                        }
                        if let Some(stroke_color) = &stroke {
                            let source = Source::Solid(SolidSource {
                                r: stroke_color.0,
                                g: stroke_color.1,
                                b: stroke_color.2,
                                a: stroke_color.3,
                            });
                            draw_target.stroke(
                                &path,
                                &source,
                                &StrokeStyle {
                                    width: stroke_weight,
                                    cap: stroke_cap,
                                    join: stroke_join,
                                    ..StrokeStyle::default()
                                },
                                &DrawOptions::default(),
                            );
                        }
                    }
                    _ => {
                        print!("{}", reading);
                    }
                },
                _ => {
                    print!("{}", reading);
                }
            }
        }

        // clear the reading so that we can read the next line
        // reading only contains one line at a time
        reading.clear();
    }

    if process.is_some() {
        process.unwrap().kill().expect("failed to kill process");
    }
}
