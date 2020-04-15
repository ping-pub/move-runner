use crate::{commands::Command, config::Config, Parameter,
            println_color, runner::MoveRunner};

pub struct CompileCommand{}

impl Command for CompileCommand{
    fn execute(&self, params: Parameter) {
        if let Parameter::Compile { home, mut source_path, module } = params {
            let cfg = Config::load_config(home);
            let mut m_runner = MoveRunner::new(cfg.clone());

            if module {
                if !source_path.exists() {
                    source_path = cfg.module_dir().join(source_path);
                }
                println_color("Compiling");
                print!("{:?}\n", &source_path.display());
                m_runner.complie_module(&source_path);
            } else {
                if !source_path.exists() {
                    source_path = cfg.script_dir().join(source_path);
                }
                println_color("Compiling");
                print!("{:?}\n", &source_path.display());
                m_runner.complie_script(&source_path);
            }

            println!("Compile finished.");
        }
    }
}



