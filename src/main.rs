pub mod cli;
use crate::cli::CLIArgs;
use oxt_confidence::Confidence;
use clap::Parser;
fn main() {
    let args = CLIArgs::parse();
    let mut conf = Confidence::new(args.ticks,args.seconds,args.time as i64,args.end_time as i64,args.rate,args.start, args.previous, args.interval, args.end);
    conf.calculate();
    print!("{}", conf.results);
}