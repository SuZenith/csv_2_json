use clap::Parser;

#[derive(Debug, Parser)]
pub struct PasswordOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}
