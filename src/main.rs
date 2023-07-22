use clap::Parser;

mod canvas;
mod cli;
mod commands;

const DEFAULT_WIDTH: usize = 800;
const DEFAULT_HEIGHT: usize = 800;

fn main() {
    let args = cli::GraphicsArgs::parse();

    let command_name = args.name.as_str();

    let canvas_height = args.height.unwrap_or(DEFAULT_HEIGHT);
    let canvas_width = args.width.unwrap_or(DEFAULT_WIDTH);

    let mut canvas = canvas::Canvas::new("Graphics", canvas_width, canvas_height);

    match command_name {
        "red_screen" => commands::full_red::RedScreen::run(&mut canvas),
        _ => println!("Unknown command"),
    }
}

trait Command {
    fn run(canvas: &mut canvas::Canvas);
}
