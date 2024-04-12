mod arguments;

use arguments::Args;
use clap::Parser;
use notify_rust::Notification;

fn main() {
    let args: Args = arguments::Args::parse();

    Notification::new()
    .summary(&args.summary)
    .body(&args.body.as_str())
    .show().unwrap();
}
