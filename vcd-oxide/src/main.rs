use clap::Parser;
use std::{fs, path::PathBuf};
use vcd_oxide_parser::ValueChangeDump;
use vcd_oxide_wavejson::WaveJson;

#[derive(Parser, Debug)]
#[command(author, version, about)]
// #[command(arg_required_else_help(true))]
struct Args {
    file: PathBuf,
    #[arg(short, long, help = "Expand busses into individual signals", default_value = "false")]
    expand_busses: bool,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let Args {
        file,
        expand_busses,
    } = args;
    let content = fs::read_to_string(&file).unwrap();
    let vcd = ValueChangeDump::parse(&content);

    let mut output_path = file;
    output_path.set_extension("json");

    let wave = WaveJson::from_vcd(vcd, expand_busses);
    let json = wave.to_json();
    fs::write(output_path, json)
}