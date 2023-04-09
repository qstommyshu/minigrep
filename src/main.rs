use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1); //1 tells the program is exited becaue of error
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    //only need to match the condition where run(config) produces error
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}"); //eprintln! so that it can't be redirected to file
        process::exit(1);
    };
}
