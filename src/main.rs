use std::time::{SystemTime, UNIX_EPOCH};
use clap::Parser;
use crate::greed::Cli;
use crate::mfa::{call_http, check_token, guess_question};

mod greed;
mod mfa;

fn main() {
    let greed = Cli::parse();

    // if greed.check {
    //     check_token(&greed);
    // } else {
    //     call_http(&greed);
    // }

    guess_question(&greed);
}
