mod constants;
mod converter;
mod path_parser;
mod scene_builder;
mod subpath_converter;
mod transform;
mod transformers;
mod types;
mod utils;
mod visitors;
mod xml_builder;

use clap::Parser;

#[derive(Parser)]
#[command(override_usage = "convert [OPTIONS] <input.lac> [output.lbrn2]")]
struct Cli {
    #[arg(
        long,
        help = "Shift all shapes so the scene bounding box starts at origin"
    )]
    normalize: bool,
    input: String,
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let output = cli.output.unwrap_or_else(|| {
        let lower = cli.input.to_lowercase();
        if lower.ends_with(".lac") {
            format!("{}.lbrn2", &cli.input[..cli.input.len() - 4])
        } else {
            format!("{}.lbrn2", cli.input)
        }
    });
    if let Err(e) = converter::Converter::new(cli.input, output, cli.normalize).run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
