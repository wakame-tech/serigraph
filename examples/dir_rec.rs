use std::path::Path;

use anyhow::Result;
use clap::Parser;
use ignore::Walk;

#[derive(Parser, Debug)]
struct Args {
    pub dir: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let current_dir = Path::new(&args.dir);
    assert!(current_dir.is_dir());

    for entry in Walk::new(current_dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some("md".as_ref()) {
            println!("{}", path.display());
        }
    }

    Ok(())
}
