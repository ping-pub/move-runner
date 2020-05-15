use std::{ io::Write, path::PathBuf };
use structopt::StructOpt;
use termcolor::{ Color, ColorChoice, ColorSpec, StandardStream, WriteColor };

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
        /// Type_arguments to run script, ie: U64, Address
        #[structopt(short, long)]
        type_args: Vec<String>,
        /// Args assigned to move script.      
        #[structopt(name = "args")]
        args: Vec<String>,
    },
    /// Compile single script/module only, use 'move build' if your source code has dependency.
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
    /// Compile and run script
    Test {
        /// Specify the home directory for new project.
        #[structopt(short, long, default_value = ".", parse(from_os_str))]
        home: PathBuf,
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
        Parameter::Test { .. } => commands::test_command(),
        //_ => panic!("unimplement"),
    };
    cmd.execute(params);
}

pub fn println_color(content: &'static str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true));
    let _ = write!(&mut stdout, "{:>12} ", content);
    let _ = stdout.reset();
}


#[test]
fn test_println() {
    println_color("Grean");
}
