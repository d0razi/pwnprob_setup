use std::process;

use colored::*;
use pwnprob_setup::gen_docker_compose;
use pwnprob_setup::gen_dockerfile;
use pwnprob_setup::gen_flag;
use pwnprob_setup::gen_run;
use pwnprob_setup::parse_config;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("{}", format!("Usage: {} <port> <docker-container-name>", args[0]).bold().red());
        process::exit(1);
    }
    
    let config = parse_config(&args);

    gen_docker_compose(&config.port, &config.container_name);
    gen_dockerfile(&config.port);
    gen_flag();
    gen_run();
}
