#![allow(unused)]
#![feature(let_else)]

use std::path::PathBuf;

use clap::Parser;

mod parser;
mod tokenizer;

#[derive(Parser)]
struct Args {
    file: PathBuf,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    Ok(())
}
