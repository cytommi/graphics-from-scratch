use clap::Parser;

mod canvas;
mod cli;
mod opts;

fn main() {
    let args = cli::GraphicsArgs::parse();

    let command_name = args.name.as_str();

    match command_name {
        "red_screen" => println!("Red screen"),
        _ => println!("Unknown command"),
    }
}
