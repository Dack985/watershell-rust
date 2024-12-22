mod watershell;

use watershell::Watershell;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug = false;
    let mut promisc = false;
    let mut tcp_mode = false;
    let mut port: u16 = 53;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-p" => promisc = true,
            "-t" => tcp_mode = true,
            "-h" => {
                println!(
                    "Usage: {} [-l port] [-p] [-t] -i iface",
                    args[0]
                );
                process::exit(0);
            }
            "-l" => {
                if i + 1 >= args.len() {
                    eprintln!("Missing value for -l");
                    process::exit(1);
                }
                port = args[i + 1].parse().unwrap_or_else(|_| {
                    eprintln!("Invalid port value");
                    process::exit(1);
                });
                i += 1;
            }
            _ => {
                eprintln!(
                    "Usage: {} [-l port] [-p] [-t] -i iface",
                    args[0]
                );
                process::exit(1);
            }
        }
        i += 1;
    }

    println!("Starting Watershell on port {}", port);

    let mut watershell = Watershell::new(port, debug, promisc, tcp_mode);
    watershell.init();

    loop {
        watershell.run_once();
    }
}
