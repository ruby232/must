use std::env;

mod server;
mod client;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args {
        if arg == "-S" {
            server::main().unwrap();
        }
        if arg == "-p" {
            client::send(String::from("pause\n")).unwrap();
        }
    }

}
