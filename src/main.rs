use anyhow::Result;
use clap::Parser;

use bril_rs::Program;

#[derive(Parser, Debug)]
struct Args {
    input: String,
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);
    let input = args.input;
    let input = std::fs::read_to_string(input).unwrap();
    let program: Program = serde_json::from_str(input.as_str()).unwrap();
    program.functions.iter().for_each(|f| {
        println!("{:?}", f.name);
        println!("{:?}", f.args);
        f.instrs.iter().for_each(|i| {
            println!("{:?}", i);
        });
    });
    Ok(())
}
