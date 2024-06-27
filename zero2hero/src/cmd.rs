use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("./base.yaml"))]
    pub config_path: String,
}

pub fn parse() -> Args {
    Args::parse()
}