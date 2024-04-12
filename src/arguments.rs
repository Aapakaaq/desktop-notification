use clap::{Parser};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long)]
    pub summary: String,
    #[arg(short, long)]
    pub body: String,
    #[arg(short, long, default_value_t = 5000)]
    pub timeout_ms: i32,
}