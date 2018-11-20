#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate json;

extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate serde_json;

extern crate rusptlib;

mod repl;
mod server;

use repl::*;
use server::*;

use std::env;

enum RunMode {
    Repl,
    Server,
}

fn main() {
    let mut args = env::args().skip(1);

    let mut run_mode = None;

    let mut server_addr = None;

    loop {
        let arg = args.next();

        match arg {
            Some(arg) => match arg.as_str() {
                "--repl" => run_mode = Some(RunMode::Repl),

                "--server" => run_mode = Some(RunMode::Server),
                "--addr" => match args.next() {
                    Some(addr) => server_addr = Some(addr),
                    None => panic!("--addr requires an address"),
                },

                _ => panic!("Unknown option {:?}", arg),
            },
            None => break,
        };
    }

    match run_mode {
        None | Some(RunMode::Repl) => repl(),
        Some(RunMode::Server) => {
            let addr = match server_addr {
                None => String::from("127.0.0.1:8081"),
                Some(addr) => addr,
            };

            server(addr)
        }
    }
}
