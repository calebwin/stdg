// extern crate sdl2;

use std::str::FromStr;

use std::io;
use std::io::BufRead;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::process;

use sdl2::render::Renderer;
use sdl2::Sdl;

// extern crate cairo;

// extern crate piston_window;

// use piston_window::*;
// use piston_window::rectangle::*;

// use std::sync::mpsc::{Sender, Receiver};
// use std::sync::mpsc;
// use std::thread;

// extern crate cairo;

// use std::env;
// use std::fs::File;
// use cairo::{Context, Format, ImageSurface};

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
					let r = instruction.params[0].parse::<u8>().unwrap();
					let g = instruction.params[1].parse::<u8>().unwrap();
					let b = instruction.params[2].parse::<u8>().unwrap();

					// create color
					let mut color = sdl2::pixels::Color::RGB(r, g, b);

					// set color
					let _ = self.ren.set_draw_color(color);
				}
			}
            InstructionType::FillSquare => {
                if instruction.params.len() == 4 {
                    // get parameters for rendering square
                    let x = instruction.params[0].parse::<i32>().unwrap();
                    let y = instruction.params[1].parse::<i32>().unwrap();
                    let size = instruction.params[2].parse::<u32>().unwrap();

                    // create square
                    let mut rect = Rect::new(x, y, size, size);

                    // render square
                    let _ = self.ren.fill_rect(rect);
                }
            }
            InstructionType::OutlineSquare => {
                if instruction.params.len() == 4 {
                    // get parameters for rendering square
                    let x = instruction.params[0].parse::<i32>().unwrap();
                    let y = instruction.params[1].parse::<i32>().unwrap();
                    let size = instruction.params[2].parse::<i32>().unwrap();

                    // create points
                    let mut top_left = Point::new(x, y);
                    let mut top_right = Point::new(x + size, y);
                    let mut bottom_left = Point::new(x, y + size);
                    let mut bottom_right = Point::new(x + size, y + size);

                    // render rectangle
                    let _ = self.ren.draw_line(top_left, top_right);
                    let _ = self.ren.draw_line(top_right, bottom_right);
                    let _ = self.ren.draw_line(bottom_right, bottom_left);
                    let _ = self.ren.draw_line(bottom_left, top_left);
                }
            }
			InstructionType::FillRect => {
				if instruction.params.len() == 4 {
					// get parameters for rendering square
					let x = instruction.params[0].parse::<i32>().unwrap();
					let y = instruction.params[1].parse::<i32>().unwrap();
					let w = instruction.params[2].parse::<u32>().unwrap();
					let h = instruction.params[3].parse::<u32>().unwrap();

					// create square
					let mut rect = Rect::new(x, y, w, h);

					// render square
					let _ = self.ren.fill_rect(rect);
				}
			}
            InstructionType::OutlineRect => {
                if instruction.params.len() == 4 {
                    // get parameters for rendering square
                    let x = instruction.params[0].parse::<i32>().unwrap();
                    let y = instruction.params[1].parse::<i32>().unwrap();
                    let w = instruction.params[2].parse::<i32>().unwrap();
                    let h = instruction.params[3].parse::<i32>().unwrap();

                    // create points
                    let mut top_left = Point::new(x, y);
                    let mut top_right = Point::new(x + w, y);
                    let mut bottom_left = Point::new(x, y + h);
                    let mut bottom_right = Point::new(x + w, y + h);

                    // render rectangle
                    let _ = self.ren.draw_line(top_left, top_right);
                    let _ = self.ren.draw_line(top_right, bottom_right);
                    let _ = self.ren.draw_line(bottom_right, bottom_left);
                    let _ = self.ren.draw_line(bottom_left, top_left);
                }
            }
            InstructionType::Line => {
                if instruction.params.len() == 4 {
                    // get parameters for rendering square
                    let x_1 = instruction.params[0].parse::<i32>().unwrap();
                    let y_1 = instruction.params[1].parse::<i32>().unwrap();
                    let x_2 = instruction.params[2].parse::<i32>().unwrap();
                    let y_2 = instruction.params[3].parse::<i32>().unwrap();

                    // create points
                    let mut start = Point::new(x_1, y_1);
                    let mut end = Point::new(x_2, y_2);

                    // render line
                    let _ = self.ren.draw_line(start, end);
                }
            }
            InstructionType::FillCircle => {
                if instruction.params.len() == 3 {
                    // get parameters for rendering square
                    let center_x = instruction.params[0].parse::<i32>().unwrap();
                    let center_y = instruction.params[1].parse::<i32>().unwrap();
                    let radius = instruction.params[2].parse::<i32>().unwrap();

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
                }
            }
            InstructionType::OutlineCircle => {
                if instruction.params.len() == 3 {
                    // get parameters for rendering square
                    let center_x = instruction.params[0].parse::<i32>().unwrap();
                    let center_y = instruction.params[1].parse::<i32>().unwrap();
                    let radius = instruction.params[2].parse::<i32>().unwrap();

                    let diameter = radius * 2;

                    let mut x = radius - 1;
                    let mut y = 0;
                    let mut tx = 1;
                    let mut ty = 1;
                    let mut error = tx - diameter;

                    while x >= y {
                        // create points
                        let mut points = vec![
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
                }
            }
            InstructionType::Point => {
                if instruction.params.len() == 2 {
                    // get parameters for rendering square
                    let x = instruction.params[0].parse::<i32>().unwrap();
                    let y = instruction.params[1].parse::<i32>().unwrap();

                    // create point
                    let mut rect = Point::new(x, y);

                    // render point
                    let _ = self.ren.draw_point(rect);
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
			            _ => {}
			        }
				}
			}
			_ => {}
		}
	}

	fn run(&mut self) {
		// get reader of instructions through standard input
		let mut instruction_reader = io::stdin();

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
				"color" => InstructionType::Color,
                "fill" => match tokens[1].as_ref() {
                    "rect" => InstructionType::FillRect,
                    "circle" => InstructionType::FillCircle,
                    _ => InstructionType::Nothing,
                },
                "outline" => match tokens[1].as_ref() {
                    "rect" => InstructionType::OutlineRect,
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
    // create contexts for rendering
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    // create new window for context
    // initalize window with default title and dimensions
    let window = match video_ctx
        .window("Untitled", 400, 400)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err),
    };

    // create renderer for rendering graphics on window
    let mut renderer = match window.renderer().build() {
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


// fn main() {
// // // 	// // channel for sending instructions
// // // 	// let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

// // // 	// // thread for reading instructions
// // // 	// let reader = thread::spawn(move || {
// // // 	// 	// loop through input from stdin
// // // 	//     for raw_line in io::stdin().lock().lines() {
// // // 	//     	// parse line for instructions
// // // 	//     	let line = raw_line.unwrap();
// // // 	//     	let mut tokens = line.split_whitespace();

// // // 	//     	// send instructions to printer
// // // 	//     	tx.clone().send(tokens.next().unwrap().to_string()).unwrap();
// // // 	//     }
// // // 	// });

// // // 	// // thread for printing graphics
// // // 	// let printer = thread::spawn(move || {
// // // 	// 	// create window with blank title and default dimensions
// // // 	// 	let mut window: PistonWindow = WindowSettings::new("", [400, 400]).build().unwrap();

// // // 	// 	while let Some(event) = window.next() {
// // // 	//     	window.draw_2d(&event, |context, graphics, _device| {
// // //  //    		    if let Ok(instruction) = rx.recv() {
// // // 	// 				print!("{}", instruction);
// // // 	// 		    }

// // // 	//     		clear([1.0; 4], graphics);
// // // 	//             rectangle([1.0, 0.0, 0.0, 1.0], // red
// // // 	//                       [0.0, 0.0, 100.0, 100.0],
// // // 	//                       context.transform,
// // // 	//                       graphics);
// // // 	//     	});
// // // 	//     }
// // // 	// });

// // // 	// reader.join();
// // // 	// printer.join();

// // 	// create window
// // 	let mut window: PistonWindow = WindowSettings::new("", [400, 400]).build().unwrap();

// // 	// loop through input from stdin
// //     for raw_line in io::stdin().lock().lines() {
// //     	// parse line
// //     	let line = raw_line.unwrap();
// //     	let mut tokens = line.split_whitespace();

// //     	// keep track of state
// //     	let mut color: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

// //     	// handle instruction
// //         match tokens.next().unwrap() {
// //         	"color" => {
// //         		let r: f32 = tokens.next().expect("expected amount of red in color").parse().expect("expected a floating point number");
// //         		let g: f32 = tokens.next().expect("expected amount of green in color").parse().expect("expected a floating point number");
// //         		let b: f32 = tokens.next().expect("expected amount of blue in color").parse().expect("expected a floating point number");
// //         		let t: f32 = tokens.next().expect("expected amount of transparency").parse().expect("expected a floating point number");

// //         		color = [r / 255f32, g / 255f32, b / 255f32, t / 255f32];
// //         	}
// //         	"clear" => {
// // 				if let Some(event) = window.next() {
// // 			        window.draw_2d(&event, |context, graphics, _device| {
// // 			        	clear([0.0, 0.0, 0.0, 1.0], graphics);
// // 			        });
// // 			    }
// //         	}
// //         	"rect" => {
// //         		let x: f64 = tokens.next().expect("expected x position of rectangle").parse().expect("expected a floating point number");
// //         		let y: f64 = tokens.next().expect("expected y position of rectangle").parse().expect("expected a floating point number");
// //         		let w: f64 = tokens.next().expect("expected width of rectangle").parse().expect("expected a floating point number");
// //         		let h: f64 = tokens.next().expect("expected height of rectangle").parse().expect("expected a floating point number");

// //        //  		println!("5");

// //        //  		while let fn(piston_window::Loop) -> piston_window::Event {piston_window::Event::Loop} = Event::Loop {
// // 			    //     window.draw_2d(&event, |context, graphics, _device| {
// // 			    //         rectangle(color,
// // 			    //                   [x, y, w, h],
// // 			    //                   context.transform,
// // 			    //                   graphics);
// // 			    //     });
// // 			    // }

// // 			    let rectangle = Rectangle::new(color::BLACK);
// // 				let dims = square(0.0, 0.0, 10.0);
// // 				rectangle.draw(dims, &draw_state::DrawState::new_alpha(), transform, gfx_graphics::GfxGraphics::);
// //         	}
// //         	_ => {
// //         		println!("bye!");
// //         	}
// //         }
// //     }

// 	// create window
// 	let mut window: PistonWindow = WindowSettings::new("", [400, 400]).build().unwrap();


// 	// keep track of some stuff
// 	let mut color: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

// 	// loop
// 	while let Some(e) = window.next() {

// 		// parse a line of input
//     	let mut line = String::new();
//     	std::io::stdin().read_line(&mut line).unwrap();
//     	let mut tokens = line.split_whitespace();

//     	match tokens.next().unwrap().as_ref() {
//     		"color" => {
//         		let r: f32 = tokens.next().expect("expected amount of red in color").parse().expect("expected a floating point number");
//         		let g: f32 = tokens.next().expect("expected amount of green in color").parse().expect("expected a floating point number");
//         		let b: f32 = tokens.next().expect("expected amount of blue in color").parse().expect("expected a floating point number");
//         		let t: f32 = tokens.next().expect("expected amount of transparency").parse().expect("expected a floating point number");

//         		color = [r / 255f32, g / 255f32, b / 255f32, t / 255f32];
//         	}
//         	"clear" => {
//         		window.draw_2d(&e, |c, g, _| {
//         			clear(color, g);
//         		});
//         	}
//     		"rect" => {
// 				let x: f64 = tokens.next().expect("expected x position of rectangle").parse().expect("expected a floating point number");
//     			let y: f64 = tokens.next().expect("expected y position of rectangle").parse().expect("expected a floating point number");
//     			let w: f64 = tokens.next().expect("expected width of rectangle").parse().expect("expected a floating point number");
//     			let h: f64 = tokens.next().expect("expected height of rectangle").parse().expect("expected a floating point number");

// 				window.draw_2d(&e, |c, g, _| {
//         			rectangle([1.0, 0.0, 0.0, 1.0], [x, y, w, h], c.transform, g);
// 				});
//     		}
//     		"present" => {
//     			// TODO present
//     		}
//     		_ => {}
//     	}
//     }
// }


// fn main() {
// 	let mut app = simple::Window::new("hello world", 1920, 1080);

// 	app.set_color(255, 0, 255, 255);
// 	app.draw_rect(simple::Rect::new(
// 	    100,
// 	    110,
// 	    120,
// 	    130,
// 	));

// 	while app.next_frame() {}
// }

// pm/executive
// - change supported platforms without changing development environment
// - change development environment without changing supported platforms
// - less coding
// - only 1 tool for employees to learn for all projects

// programmer
// - print text
// - print graphics