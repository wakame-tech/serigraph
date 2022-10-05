use std::{ops::Range, path::Path};

use anyhow::Result;
use clap::Parser;

use crate::book::Book;

pub mod book;

#[derive(Parser, Debug)]
pub struct Args {
    pub input_path: String,

    #[clap(short = 'O', long)]
    pub output_path: String,

    #[clap(long)]
    pub begin: Option<usize>,

    #[clap(long)]
    pub end: Option<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input_path = Path::new(&args.input_path);
    let output_path = Path::new(&args.output_path);

    let mut book = Book::from_path(input_path)?;
    let range = args.begin.unwrap_or(0)..args.end.unwrap_or(book.graph.node_count());
    println!("{}", book);
    book.export_as_mdbook(output_path, range)?;
    Ok(())
}
