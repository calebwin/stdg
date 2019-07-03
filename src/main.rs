// use standard library
use std::io;
use std::io::BufRead;
use std::process;

// use sdl2
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::Renderer;
use sdl2::Sdl;

/// Executes instructions to draw
trait Executor {
	fn execute(&mut self, instruction: &Instruction);
	fn run(&mut self);
}

/// Engine to execute instructions to draw graphics
struct Engine<'a> {
    ctx: Sdl,
    ren: Renderer<'a>,
}

/// Implementation of creation of engines
impl<'a> Engine<'a> {
	fn from(ctx: Sdl, ren: Renderer) -> Engine {
		Engine {
			ctx: ctx,
			ren: ren
		}
	}
}

/// Implementation of execution of instructions on engine
impl<'a> Executor for Engine<'a> {
	fn execute(&mut self, instruction: &Instruction) {
		// execute instruction based on its type
		match instruction.ty {
			InstructionType::Color => {
				if instruction.params.len() == 3 {
					// get parameters for setting color
					let r = instruction.params[0].parse::<u8>().expect("expected unsigned 8-bit integer as value of red in color");
					let g = instruction.params[1].parse::<u8>().expect("expected unsigned 8-bit integer as value of green in color");
					let b = instruction.params[2].parse::<u8>().expect("expected unsigned 8-bit integer as value of blue in color");

					// create color
					let color = sdl2::pixels::Color::RGB(r, g, b);

					// set color
					let _ = self.ren.set_draw_color(color);
				} else {
                    panic!("incorrect number of tokens found on line that begins as a command to stdg");
                }
			}
            InstructionType::FillSquare => {
                if instruction.params.len() == 4 {
                    // get parameters for rendering square
                    let x = instruction.params[0].parse::<i32>().expect("expected 32-bit integer as value of x position of square");
                    let y = instruction.params[1].parse::<i32>().expect("expected 32-bit integer as value of y position of square");
                    let size = instruction.params[2].parse::<u32>().expect("expected unsigned 32-bit integer as value size of square");

                    // create square
                    let rect = Rect::new(x, y, size, size);

                    // render square
                    let _ = self.ren.fill_rect(rect);
                } else {
                    panic!("incorrect number of tokens found on line that begins as a command to stdg");
                }
            }
            InstructionType::OutlineSquare => {
                if instruction.params.len() == 4 {
                    // get parameters for rendering square
                    let x = instruction.params[0].parse::<i32>().expect("expected 32-bit integer as value of x position of square");
                    let y = instruction.params[1].parse::<i32>().expect("expected 32-bit integer as value of y position of square");
                    let size = instruction.params[2].parse::<i32>().expect("expected 32-bit integer as value of size of square");

                    // create points
                    let top_left = Point::new(x, y);
                    let top_right = Point::new(x + size, y);
                    let bottom_left = Point::new(x, y + size);
                    let bottom_right = Point::new(x + size, y + size);

                    // render rectangle
                    let _ = self.ren.draw_line(top_left, top_right);
                    let _ = self.ren.draw_line(top_right, bottom_right);
                    let _ = self.ren.draw_line(bottom_right, bottom_left);
                    let _ = self.ren.draw_line(bottom_left, top_left);
                } else {
                    panic!("incorrect number of tokens found on line that begins as a command to stdg");
                }
            }
			InstructionType::FillRect => {
				if instruction.params.len() == 4 {
					// get parameters for rendering square
					let x = instruction.params[0].parse::<i32>().expect("expected 32-bit integer as value of x position of rectangle");
					let y = instruction.params[1].parse::<i32>().expect("expected 32-bit integer as value of y position of rectangle");
					let w = instruction.params[2].parse::<u32>().expect("expected unsigned 32-bit integer as value of width of rectangle");
					let h = instruction.params[3].parse::<u32>().expect("expected unsigned 32-bit integer as value of height of rectangle");

					// create square
					let rect = Rect::new(x, y, w, h);

					// render square
					let _ = self.ren.fill_rect(rect);
				} else {
                    panic!("incorrect number of tokens found on line that begins as a command to stdg");
                }
			}
            InstructionType::OutlineRect => {
                if instruction.params.len() == 4 {
                    // get parameters for rendering square
                    let x = instruction.params[0].parse::<i32>().expect("expected 32-bit integer as value of x position of rectangle");
                    let y = instruction.params[1].parse::<i32>().expect("expected 32-bit integer as value of y position of rectangle");
                    let w = instruction.params[2].parse::<i32>().expect("expected 32-bit integer as value of width of rectangle");
                    let h = instruction.params[3].parse::<i32>().expect("expected 32-bit integer as value of height of rectangle");

                    // create points
                    let top_left = Point::new(x, y);
                    let top_right = Point::new(x + w, y);
                    let bottom_left = Point::new(x, y + h);
                    let bottom_right = Point::new(x + w, y + h);

                    // render rectangle
                    let _ = self.ren.draw_line(top_left, top_right);
                    let _ = self.ren.draw_line(top_right, bottom_right);
                    let _ = self.ren.draw_line(bottom_right, bottom_left);
                    let _ = self.ren.draw_line(bottom_left, top_left);
                } else {
                    panic!("incorrect number of tokens found on line that begins as a command to stdg");
                }
            }
            InstructionType::Line => {
                if instruction.params.len() == 4 {
                    // get parameters for rendering square
                    let x_1 = instruction.params[0].parse::<i32>().expect("expected 32-bit integer as value of x position of start of line");
                    let y_1 = instruction.params[1].parse::<i32>().expect("expected 32-bit integer as value of y position of start of line");
                    let x_2 = instruction.params[2].parse::<i32>().expect("expected 32-bit integer as value of x position of end of line");
                    let y_2 = instruction.params[3].parse::<i32>().expect("expected 32-bit integer as value of y position of end of line");

                    // create points
                    let start = Point::new(x_1, y_1);
                    let end = Point::new(x_2, y_2);

                    // render line
                    let _ = self.ren.draw_line(start, end);
                } else {
                    panic!("incorrect number of tokens found on line that begins as a command to stdg");
                }
            }
            InstructionType::FillCircle => {
                if instruction.params.len() == 3 {
                    // get parameters for rendering square
                    let center_x = instruction.params[0].parse::<i32>().expect("expected 32-bit integer as value of x position of center of circle");
                    let center_y = instruction.params[1].parse::<i32>().expect("expected 32-bit integer as value of y position of center of circle");
                    let radius = instruction.params[2].parse::<i32>().expect("expected 32-bit integer as value of radius of circle");

                    let mut points = vec![];

                    let diameter = radius * 2;

                    let mut x = radius - 1;
                    let mut y = 0;
                    let mut tx = 1;
                    let mut ty = 1;
                    let mut error = tx - diameter;

                    while x >= y {
                        // create points
                        points.push(Point::new(center_x + x, center_y - y));
                        points.push(Point::new(center_x + x, center_y + y));
                        points.push(Point::new(center_x - x, center_y - y));
                        points.push(Point::new(center_x - x, center_y + y));
                        points.push(Point::new(center_x + y, center_y - x));
                        points.push(Point::new(center_x + y, center_y + x));
                        points.push(Point::new(center_x - y, center_y - x));
                        points.push(Point::new(center_x - y, center_y + x));

                        if error <= 0 {
                            y += 1;
                            error += ty;
                            ty += 2;
                        }

                        if error > 0 {
                            x -= 1;
                            tx += 2;
                            error += tx - diameter;
                        }
                    }

                    // render points
                    for point_a in points.clone() {
                        for point_b in points.clone() {
                            let _ = self.ren.draw_line(point_a.clone(), point_b.clone());
                        }
                    }
                } else {
                    panic!("incorrect number of tokens found on line that begins as a command to stdg");
                }
            }
            InstructionType::OutlineCircle => {
                if instruction.params.len() == 3 {
                    // get parameters for rendering square
                    let center_x = instruction.params[0].parse::<i32>().expect("expected 32-bit integer as value of x position of center of circle");
                    let center_y = instruction.params[1].parse::<i32>().expect("expected 32-bit integer as value of y position of center of circle");
                    let radius = instruction.params[2].parse::<i32>().expect("expected 32-bit integer as value of radius of circle");

                    let diameter = radius * 2;

                    let mut x = radius - 1;
                    let mut y = 0;
                    let mut tx = 1;
                    let mut ty = 1;
                    let mut error = tx - diameter;

                    while x >= y {
                        // create points
                        let points = vec![
                            Point::new(center_x + x, center_y - y),
                            Point::new(center_x + x, center_y + y),
                            Point::new(center_x - x, center_y - y),
                            Point::new(center_x - x, center_y + y),
                            Point::new(center_x + y, center_y - x),
                            Point::new(center_x + y, center_y + x),
                            Point::new(center_x - y, center_y - x),
                            Point::new(center_x - y, center_y + x),
                        ];

                        // render points
                        for point in points {
                            let _ = self.ren.draw_point(point);
                        }

                        if error <= 0 {
                            y += 1;
                            error += ty;
                            ty += 2;
                        }

                        if error > 0 {
                            x -= 1;
                            tx += 2;
                            error += tx - diameter;
                        }
                    }
                } else {
                    panic!("incorrect number of tokens found on line that begins as a command to stdg");
                }
            }
            InstructionType::Point => {
                if instruction.params.len() == 2 {
                    // get parameters for rendering square
                    let x = instruction.params[0].parse::<i32>().expect("expected 32-bit integer as value of x position of point");
                    let y = instruction.params[1].parse::<i32>().expect("expected 32-bit integer as value of y position of point");

                    // create point
                    let point = Point::new(x, y);

                    // render point
                    let _ = self.ren.draw_point(point);
                } else {
                    panic!("incorrect number of tokens found on line that begins as a command to stdg");
                }
            }

			InstructionType::Clear => {
				// clear
				let _ = self.ren.clear();
			}
			InstructionType::Present => {
				// clear
				let _ = self.ren.present();
			}
			InstructionType::Handle => {
				// get pending events
				let mut events = self.ctx.event_pump().unwrap();

				// handle all pending events
				for event in events.poll_iter() {
			        match event {
			            Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
			                process::exit(1);
			            }
                        Event::KeyDown {keycode: Some(key_code), ..} => {
                            println!("key down");
                            println!("{:?}", key_code);
                        }
                        Event::KeyUp {keycode: Some(key_code), ..} => {
                            println!("key up");
                            println!("{:?}", key_code);
                        }
                        Event::MouseMotion {x, y, ..} => {
                            println!("mouse motion");
                            println!("{:?}", x);
                            println!("{:?}", y);
                        }
                        Event::FingerDown {..} => {
                            println!("finger down");
                        }
                        Event::FingerUp {..} => {
                            println!("finger up");
                        }
                        Event::FingerMotion {x, y, ..} => {
                            println!("finger motion");
                            println!("{:?}", x);
                            println!("{:?}", y);
                        }
			            _ => {}
			        }
				}
			}
			_ => {}
		}
	}

	fn run(&mut self) {
		// get reader of instructions through standard input
		let instruction_reader = io::stdin();

		// keep executing instructions
		// the only way this loop might be ended when a "handle" instruction is executed
		loop {
			// execute all instructions
			for raw_instruction in instruction_reader.lock().lines() {
				// parse instruction into its type and its parameters
				let instruction = Instruction::from(raw_instruction.unwrap());

				// execute the instruction
				self.execute(&instruction);
			}
		}
	}
}

/// Represents an instrution to be executed by rendering engine
struct Instruction {
	ty: InstructionType,
	params: Vec<String>
}

/// Types of instructions that can be executed
enum InstructionType {
	Nothing,
    Resize,
    Title,
	Color,
    FillSquare,
	FillRect,
    FillCircle,
    OutlineSquare,
    OutlineRect,
    OutlineCircle,
    Point,
    Line,
	Clear,
	Present,
	Handle,
}

/// Implementation of creation of instructions from source
impl Instruction {
	fn from(source: String) -> Instruction {
		// default values of instruction
		let mut ty = InstructionType::Nothing;
		let mut params = Vec::new();

		// tokenize source
		let tokens = source.split_whitespace().map(|parameter| parameter.to_string()).collect::<Vec<String>>();

		// get instruction type
		if tokens.len() >= 1 {
			ty = match tokens[0].as_ref() {
                "resize" => InstructionType::Resize,
                "title" => InstructionType::Title,
				"color" => InstructionType::Color,
                "fill" => match tokens[1].as_ref() {
                    "rect" => InstructionType::FillRect,
                    "square" => InstructionType::FillSquare,
                    "circle" => InstructionType::FillCircle,
                    _ => InstructionType::Nothing,
                },
                "outline" => match tokens[1].as_ref() {
                    "rect" => InstructionType::OutlineRect,
                    "square" => InstructionType::OutlineSquare,
                    "circle" => InstructionType::OutlineCircle,
                    _ => InstructionType::Nothing,
                },
                "point" => InstructionType::Point,
                "line" => InstructionType::Line,
				"clear" => InstructionType::Clear,
				"present" => InstructionType::Present,
				"handle" => InstructionType::Handle,
				_ => InstructionType::Nothing,
			}
		}

		// get parameters
		if tokens.len() >= 2 {
			params = match tokens[0].as_ref() {
                "fill" | "outline" => tokens.clone()[2..].to_vec(),
                _ => tokens.clone()[1..].to_vec()
            };
		}

		// return the new instruction
		Instruction {
			ty: ty,
			params: params,
		}
	}
}

fn main() {
    // get metadata
    let mut name = String::from("Untitled");
    let mut w = 400;
    let mut h = 400;

    let stdin = io::stdin();
    let mut introduction_index = 0;
    for line in stdin.lock().lines() {
        match introduction_index {
            0 => {
                let tokens = line.expect("expected window command").split_whitespace().map(|token| token.to_string()).collect::<Vec<String>>();;

                if tokens.len() == 3 {
                    if &tokens[0] == "window" {
                        w = tokens[1].parse::<u32>().expect("expected 32-bit unsigned integer");
                        h = tokens[2].parse::<u32>().expect("expected 32-bit unsigned integer");
                    } else {
                        panic!("expected window command");
                    }
                } else {
                    panic!("expected 2 arguments for window command");
                }
            }
            1 => {
                let mut title_command_line = line.expect("expected title command");

                // TODO get title
                if title_command_line.starts_with("title ") {
                    name = title_command_line.split_off(6);
                } else {
                    panic!("expected title command");
                }
            }
            _ => {}
        }

        if introduction_index >= 1 {
            break;
        } else {
            introduction_index += 1;
        }
    }

    // create contexts for rendering
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    // create new window for context
    // initalize window with default title and dimensions
    let window = match video_ctx
        .window(&name, w, h)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err),
    };

    // create renderer for rendering graphics on window
    let renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err),
    };

    // create engine for executing instructions
    // owns the context and the renderer
    let mut engine = Engine::from(ctx, renderer);
	
	// run engine
	engine.run();
}

// TODO print errors