use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::process;
use std::path::Path;
use colored::*;
use md5;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("{}", format!("Usage: {} <port> <docker-container-name>", args[0]).bold().red());
        process::exit(1);
    }
    let port = &args[1];
    let container_name = &args[2];

    gen_docker_compose(port, container_name);
    gen_dockerfile(port);
    gen_flag();
    gen_run();
}

fn gen_run() {
    let content = format!("#!/bin/sh\n
exec 2>/dev/null
timeout 120 ./prob");

    write_file("run.sh", &content);
}

fn gen_docker_compose(port: &str, container_name: &str) {
    let content = format!(
        "services:\n  prob:\n    build: .\n    ports:\n      - \"{0}:{0}\"\n    container_name: {1}",
        port, container_name
    );
    write_file("docker-compose.yml", &content);
}

fn gen_dockerfile(port: &str) {
    let content = format!(
"FROM ubuntu:22.04
ENV USER=prob
ENV PORT={0}

RUN apt-get update \
    && apt-get install -y socat

RUN adduser --disabled-password --gecos \"\" $USER \
&& chown -R root:$USER /home/$USER && chmod 750 /home/$USER

COPY --chown=root:$USER ./prob /home/$USER/prob
COPY --chown=root:$USER ./run.sh /home/$USER/run.sh
COPY --chown=root:$USER ./flag /home/$USER/flag

RUN chmod 550 /home/$USER/run.sh
WORKDIR /home/$USER

USER root
EXPOSE $PORT

CMD socat TCP-LISTEN:$PORT,reuseaddr,fork EXEC:/home/$USER/run.sh,su=$USER",
        port
    );
    write_file("Dockerfile", &content);
}

fn get_prob_md5() -> Result<String, String> {
    let path = Path::new("prob");

    if !path.exists() {
        return Err("".to_string());
    }

    let mut file = fs::File::open(path).map_err(|err| format!("Error opening file: {}", err))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|err| format!("Error reading: {}", err))?;

    let digest = md5::compute(buffer);
    Ok(format!("{:x}", digest))
}

fn gen_flag() {
    match get_prob_md5() {
        Ok(prob_hash) => {
            let flag_content = format!("d0razi{{{}}}", prob_hash);
            write_file("flag", &flag_content);
        }
        Err(_) => {
            let flag_content = format!("d0razi{{Fake}}");
            write_file("flag", &flag_content);
        }
    }
}

fn write_file(filename: &str, content: &str) {
    let mut file = File::create(filename).unwrap_or_else(|err| {
        eprintln!("{}", format!("Error creating {}: {}", filename, err).bold().red());
        process::exit(1);
    });
    file.write_all(content.as_bytes()).unwrap_or_else(|err| {
        eprintln!("{}", format!("Error writing {}: {}", filename, err).bold().red());
        process::exit(1);
    });
    println!("{} {}", "[+]".green().bold(), filename.bold());
}