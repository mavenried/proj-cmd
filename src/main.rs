use clap::Parser;
use std::fs;
mod args;
mod handlers;

use args::{Action, ProjArgs};
use handlers::*;

fn main() {
    let config_path = dirs::config_dir().unwrap().join("proj-cmd/projrc");
    if let Ok(projpath) = fs::read_to_string(&config_path) {
        let args = ProjArgs::parse();

        match args.action {
            Action::Goto(goto) => handle_goto(projpath, goto),
            Action::List(list) => handle_list(projpath, list),
            Action::Make(make) => handle_make(projpath, make),
            Action::Create(create) => handle_create(projpath, create),
            Action::Setup(setup) => handle_setup(setup),
            Action::Init(init) => handle_init(init),
            Action::Zip(zip) => handle_zip(projpath, zip),
        }
    } else {
        let home_path = dirs::home_dir().unwrap();
        let home_path = home_path.to_str().unwrap();

        println!("Cannot find config file. Creating...");
        fs::create_dir_all(dirs::config_dir().unwrap().join("proj-cmd"))
            .expect("Failed to create dirs");
        fs::write(config_path, home_path).expect("Failed to create files");
        println!("Project root set to home dir, Use proj setup <path> to update ");
    }
}
