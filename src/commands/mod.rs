use std::path::PathBuf;

use libra_config::config::{ExecutionConfig, RootPath};
use libra_types::transaction::{TransactionArgument, TransactionPayload};
use move_vm_types::values::Value;

use crate::{Parameter, println_color};
use crate::config::Config;
use crate::runner::MoveRunner;

pub mod build;
pub mod compile;
pub mod new;
pub mod run;
pub mod test;

pub trait Command {
    fn execute(&self, params: Parameter);
}

pub fn new_command() -> Box<dyn Command> {
    Box::new(new::NewCommand {})
}

pub fn compile_command() -> Box<dyn Command> {
    Box::new(compile::CompileCommand {})
}

pub fn build_command() -> Box<dyn Command> {
    Box::new(build::BuildCommand {})
}

pub fn run_command() -> Box<dyn Command> {
    Box::new(run::RunCommand {})
}

pub fn test_command() -> Box<dyn Command> {
    Box::new(test::TestCommand {})
}


fn load_genesis(cfg: &Config, runner: &mut MoveRunner) {
    println_color("Loading");
    print!("'genesis.blob' from {:?}\n", &cfg.home);
    let mut exec_cfg = ExecutionConfig::default();
    exec_cfg.genesis_file_location = PathBuf::from("genesis.blob");
    exec_cfg.load(&RootPath::new(&cfg.home)).expect("'genesis.blob' is invalid:");

    let tx = exec_cfg.genesis.unwrap();
    let gen_payload = tx.as_signed_user_txn().unwrap().payload();
    match &gen_payload {
        TransactionPayload::WriteSet(cs) => {
            runner.datastore.add_write_set(cs.write_set());
            //print_all(cs);
        },
        TransactionPayload::Module(m) => {
            println!("module:{:?}", m)
        },
        TransactionPayload::Script(s) => {
            println!("script:{:?}", s)
        },
        TransactionPayload::Program => {
            println!("unimplemented")
        },
    }
}

/// Convert the transaction arguments into move values.
fn convert_txn_args(args: &[TransactionArgument]) -> Vec<Value> {
    args.iter()
        .map(|arg| match arg {
            TransactionArgument::U64(i) => Value::u64(*i),
            TransactionArgument::Address(a) => Value::address(*a),
            TransactionArgument::Bool(b) => Value::bool(*b),
            TransactionArgument::U8Vector(v) => Value::vector_u8(v.clone()),
        })
        .collect()
}