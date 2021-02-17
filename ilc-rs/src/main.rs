use clap::{App, Arg};
use std::fs;
use zmq;

fn main() {
    let matches = App::new("Send text.")
        .version("0.1")
        .author("Abner Kaizer <abnerkaizer@protonmail.com>")
        .about("Send text content through ZeroMQ socket.")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("INPUT")
                .help("Sets the input file")
                .takes_value(true),
        )
        .get_matches();
    let filename = match matches.value_of("input") {
        Some(file) => file,
        None => "",
    };

    if filename != "" {
        let contents = match fs::read_to_string(filename) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("File not found, error: {:?}", e);
                std::process::exit(1);
            }
        };
        send_msg(contents);
    } else {
        println!("Need file name, found: {}", filename);
    }
}

fn send_msg(contents: String) {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://127.0.0.1:9090").unwrap();
    socket.send(&contents, 0).unwrap();
}
