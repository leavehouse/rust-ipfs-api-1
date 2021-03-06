extern crate ipfs_api;
extern crate tokio_core;

use ipfs_api::{response, IpfsClient};
use tokio_core::reactor::Core;

fn print_recursive(indent: usize, cmd: &response::CommandsResponse) {
    let cmd_indent = " ".repeat(indent * 4);
    let opt_indent = " ".repeat((indent + 1) * 4);

    println!("{}[{}]", cmd_indent, cmd.name);

    if cmd.options.len() > 0 {
        println!("{}* options:", cmd_indent);
        for options in cmd.options.iter() {
            println!("{}{}", opt_indent, &options.names[..].join(", "));
        }
    }

    if cmd.subcommands.len() > 0 {
        println!("{}- subcommands:", cmd_indent);
        for subcommand in cmd.subcommands.iter() {
            print_recursive(indent + 1, subcommand);
        }
    }
}

// Creates an Ipfs client, and gets a list of available commands from the
// Ipfs server.
//
fn main() {
    println!("connecting to localhost:5001...");

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());
    let req = client.commands();

    print_recursive(0, &core.run(req).expect("expected a valid response"));
}
