extern crate sdl2;

use std::str::FromStr;

use std::io;
use std::io::BufRead;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::process;

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
					let r = instruction.params[0].parse::<u8>().unwrap();
					let g = instruction.params[1].parse::<u8>().unwrap();
					let b = instruction.params[2].parse::<u8>().unwrap();

					// create color
					let mut color = sdl2::pixels::Color::RGB(r, g, b);

					// set color
					let _ = self.ren.set_draw_color(color);
				}
			}
			InstructionType::Rect => {
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
	Rect,
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
				"rect" => InstructionType::Rect,
				"clear" => InstructionType::Clear,
				"present" => InstructionType::Present,
				"handle" => InstructionType::Handle,
				_ => InstructionType::Nothing,
			}
		}

		// get parameters
		if tokens.len() >= 2 {
			params = tokens.clone()[1..].to_vec();
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
