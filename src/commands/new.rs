use crate::{config::Config, commands::Command, Parameter};

pub struct NewCommand{}

impl Command for NewCommand{
    fn execute(&self, params: Parameter) {
        if let Parameter::New{home, name} = params {
            let x = Config::new(name, home);
            x.initial();
        }
    }
}



