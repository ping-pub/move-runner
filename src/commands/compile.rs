use crate::{config::Config, commands::Command, Parameter,};

pub struct CompileCommand{}

impl Command for CompileCommand{
    fn execute(&self, params: Parameter) {
        if let Parameter::Compile{home, source_path} = params {
            let _x = Config::load_config(home);
            println!("{:?}", source_path);
        }
    }
}



