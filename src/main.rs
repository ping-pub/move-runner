use std::path::PathBuf;

use structopt::StructOpt;

mod commands;
mod config;
mod runner;

#[derive(StructOpt, Debug)]
#[structopt(name = "move")]
/// Move local runner, allows developers to compile and run Move script/modules on local.
pub enum Parameter {
    /// New a move project
    New {
        /// Specify the home directory for new project.
        #[structopt(short, long, default_value = ".", parse(from_os_str))]
        home: PathBuf,
        /// Address to run script, should start with '0x'
        #[structopt(short, long, default_value = "move-project")]
        name: String,
    },
    /// Build all modules and scripts in src directory.
    Build {
        /// Specify the home directory for new project.
        #[structopt(short, long, default_value = ".", parse(from_os_str))]
        home: PathBuf,
    },
    /// Compile and run script
    Run {
        /// Specify the home directory for new project.
        #[structopt(short, long, default_value = ".", parse(from_os_str))]
        home: PathBuf,
        /// Compile source file.      
        #[structopt(parse(from_os_str))]
        source_path: PathBuf,
        /// Args assigned to move script.      
        #[structopt(name="args")]
        args: Vec<String>,
    },
    /// Compile script/module only
    //#[structopt(help = "add files to the staging area")]
    Compile {  
        /// Specify the home directory for new project.
        #[structopt(short, long, default_value = ".", parse(from_os_str))]
        home: PathBuf,
        /// Compile source file.      
        #[structopt(parse(from_os_str))]
        source_path: PathBuf,
        /// Compile as module.
        #[structopt(short)]
        module: bool,
    },
}

fn main() {
    let params = Parameter::from_args();

    execute(params);
}

fn execute(params: Parameter) {
    let cmd: Box<dyn commands::Command> = match &params {
        Parameter::Build { .. } => commands::build_command(),
        Parameter::Run { .. } => commands::run_command(),
        Parameter::Compile { .. } => commands::compile_command(),
        Parameter::New { .. } => commands::new_command(),
        //_ => panic!("unimplement"), 
    };
    cmd.execute(params);
}


