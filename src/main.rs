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
        /// Address to run script, should start with '0x'
        #[structopt(short = "a", long = "address", default_value = "0x0")]
        address: String,
        /// Generate move source map
        #[structopt(short = "sm", long = "source-map")]
        source_map: bool,
        /// Source file to run, for example: src/scripts/hello.mvir.
        #[structopt(parse(from_os_str))]
        source_path: PathBuf,
        /// Module path.
        #[structopt(short, long, default_value = "./src/modules", parse(from_os_str))]
        module_path: PathBuf,
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
    },
}

fn main() {
    let params = Parameter::from_args();
    println!("{:?}", &params);

    execute(params);
}

fn execute(params: Parameter) {
    let cmd: Box<dyn commands::Command> = match &params {
        Parameter::Build{home:_} => commands::build_command(),
        Parameter::Compile{home:_, source_path:_} => commands::compile_command(),
        Parameter::New{home:_,name:_} => commands::new_command(),
        _ => panic!("unimplement"), 
    };
    cmd.execute(params);
}


