use bytecode_verifier::verifier::VerifiedModule;

use glob::glob;

use crate::{
    commands::Command,
    config::Config,
    Parameter,
    runner::MoveRunner,
};

pub struct BuildCommand{}

impl Command for BuildCommand{
    fn execute(&self, params: Parameter) {
        if let Parameter::Build{home} = params {
                    
            let cfg = Config::load_config(home);
            let mut runner = MoveRunner::new(cfg.clone());
            
            println!("loading modules in {}",format!("{}/**/*.mvir", &cfg.module_dir().display()));
            let mdir = glob(&format!("{}/**/*.mvir", &cfg.module_dir().display())).expect("Module directory is not valid.");
            for entry in mdir {
                match entry {
                    Ok(path) => {
                        println!("Compiling: {:?}", &path.display());
                        let m: VerifiedModule = runner.complie_module(&path);
                        let cm = &m.as_inner();
                        runner.datastore.add_module(&cm.self_id(), cm);
                    },
                    Err(_)=> {
                        panic!("Failed to load modules source file.");
                    }
                }
                
            };

            println!("\nloading scripts in {}",format!("{}/**/*.mvir", cfg.script_dir().display()));
            let sdir = glob(&format!("{}/**/*.mvir", cfg.script_dir().display())).expect("Script Directory is not valid");
            
            for entry in sdir {
                match entry {
                    Ok(path) => {
                        println!("Compiling: {:?}", path.display());
                        &runner.complie_script(&path);
                    },
                    Err(_)=> {
                        panic!("Failed to load script source file.");
                    }
                }
            };
            println!("Build finished.");
        }
    }
}
