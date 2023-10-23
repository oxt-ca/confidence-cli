use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLIArgs {
    #[arg(
        short = 's',
        long = "seconds",
        help = "How many seconds was the test period",
        default_value_t = 300.00
    )]
    pub seconds: f64,

    #[arg(
        short = 'r',
        long = "rate",
        help = "What is the return to last tick rate",
        default_value_t = 0.65
    )]
    pub rate: f64,

    #[arg(
        short = 't',
        long = "ticks",
        help = "How many ticks were in the test period",
        default_value_t = 1000.00
    )]
    pub ticks: f64,

    #[arg(
        short = 'S',
        long = "start",
        help = "What is the starting value of the security",
        default_value_t = 1000.00
    )]
    pub start: f64,

    #[arg(
        short = 'p',
        long = "previous",
        help = "What was the previous value of the security",
        default_value_t = 999.75
    )]
    pub previous: f64,

    #[arg(
        short = 'i',
        long = "interval",
        help = "What is the average interval between ticks",
        default_value_t = 0.25
    )]
    pub interval: f64,

    #[arg(
        short = 'R',
        long = "range",
        help = "How far the security traveled in the test period",
        default_value_t = 0.00
    )]
    pub range: f64,

    #[arg(
        short = 'T',
        long = "time",
        help = "Time confidence was calculated",
        default_value_t = 0
    )]
    pub time: u32,
    #[arg(
        short = 'e',
        long = "end",
        help = "Last price of security",
        default_value_t = 0.00
    )]
    pub end: f64,
    #[arg(
        short = 'E',
        long = "endTime",
        help = "Confidence to check too",
        default_value_t = 0
    )]
    pub end_time: u32,
}
