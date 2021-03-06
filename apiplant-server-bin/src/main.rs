use seahorse::{App, Context, Flag, FlagType};
use std::env;
use std::alloc::System;
use apiplant_server_lib;

#[global_allocator]
static ALLOCATOR: System = System;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(env!("CARGO_PKG_NAME"))
        .flag(Flag::new("config", "\t--config ./config.toml", FlagType::String))
        .flag(Flag::new("plugins", "\t--plugins ./plugins", FlagType::String))
        .action(run);

    app.run(args);
}

fn run(c: &Context) {
    apiplant_server_lib::init_apiplant(
        &c.string_flag("config").unwrap_or("./config.toml".to_string()),
        &c.string_flag("plugins").unwrap_or("./plugins".to_string()),
    );    
}