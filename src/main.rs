use base64_img::decode_image_data;
use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[clap(short, long)]
    input: String,

    #[clap(short, long)]
    output: String,

    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.verbose {
        eprintln!("reading from {}", args.input);
    }

    let input = std::fs::read_to_string(&args.input)?;

    let decoded = decode_image_data(&input)?;

    if args.verbose {
        eprintln!("writing to {}", args.output);
    }

    std::fs::write(&args.output, &decoded)?;

    Ok(())
}
