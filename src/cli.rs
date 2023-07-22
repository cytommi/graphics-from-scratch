pub use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct GraphicsArgs {
    pub name: String,

    pub width: Option<usize>,
    pub height: Option<usize>,
}
