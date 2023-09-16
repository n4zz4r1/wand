use clap::Parser;

// ref.: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
#[derive(Parser)]
#[command(name = "wand")]
#[command(author = "n4zz4r1 <nazzari_red@pm.me>")]
#[command(author, version, about)]
pub struct Cli {
    /// An token generated example
    #[arg(long)]
    pub hash: Option<String>,
    #[arg(long, default_value_t = false)]
    pub check: bool,
    /// Time when token was created
    #[arg(short, long)]
    pub time: Option<i128>,
    /// Keywords to run to (currently only one)
    #[arg(short, long)]
    pub keyword: Option<String>,
}
