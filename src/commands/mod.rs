
use crate::Parameter;
pub mod new;
pub mod compile;
pub mod build;
pub mod run;

pub trait Command {
    fn execute(&self, params: Parameter);
}

pub fn new_command()->Box<dyn Command> {
    Box::new(new::NewCommand{})
}

pub fn compile_command()->Box<dyn Command> {
    Box::new(compile::CompileCommand{})
}

pub fn build_command()->Box<dyn Command> {
    Box::new(build::BuildCommand{})
}

pub fn run_command()->Box<dyn Command> {
    Box::new(run::RunCommand{})
}