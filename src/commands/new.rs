use crate::{commands::Command, config::Config, Parameter};

pub struct NewCommand {}

impl Command for NewCommand {
    fn execute(&self, params: Parameter) {
        if let Parameter::New { home, name } = params {
            let x = Config::new(name.clone(), home);
            x.initial();
            x.genesis();
            println!("Move project[{}] created successfully, Move now!", name);
        }
    }
}
