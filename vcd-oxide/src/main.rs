use clap::Parser;
use std::{fs, path::PathBuf};
use vcd_oxide_parser::ValueChangeDump;
use vcd_oxide_wavejson::WaveJson;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(arg_required_else_help(true))]
struct Args {
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("Input file: {:?}", args.input);
    let content = fs::read_to_string(&args.input).unwrap();
    let vcd = ValueChangeDump::parse(&content);

    let mut output = args.input.clone();
    output.set_extension("json");
    fs::write(output, WaveJson::from(vcd).to_json()).unwrap();
}
