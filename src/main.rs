use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1); //1 tells the program is exited becaue of error
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //only need to match the condition where run(config) produces error
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}
