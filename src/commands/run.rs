use std::path::PathBuf;

use bytecode_verifier::verifier::VerifiedModule;
use libra_config::config::{ExecutionConfig, RootPath};
use libra_types::transaction::{parse_as_transaction_argument, TransactionArgument, TransactionPayload};
use move_core_types::{
    gas_schedule::{GasAlgebra, GasUnits},
};
use move_vm_runtime::MoveVM;
use move_vm_state::execution_context::{ExecutionContext, TransactionExecutionContext};
use move_vm_types::values::values_impl::Value;
use vm::{
    errors::VMResult,
    gas_schedule,
    transaction_metadata::TransactionMetadata,
};

use glob::glob;

use crate::{commands::Command, config::Config, Parameter, println_color, runner::MoveRunner};

pub struct RunCommand {}

impl Command for RunCommand {
    fn execute(&self, params: Parameter) {
        if let Parameter::Run {
            home,
            mut source_path,
            args,
        } = params
        {
            // check if arguments are valid.
            let ta_args: Vec<TransactionArgument> = args
                .iter()
                .map(|arg| parse_as_transaction_argument(arg).unwrap())
                .collect();
            let va_tags = convert_txn_args(&ta_args);

            let cfg = Config::load_config(home);
            let mut m_runner = MoveRunner::new(cfg.clone());
            println_color("Loading");
            print!("modules from {}\n", &cfg.module_dir().display());
            let mdir = glob(&format!("{}/**/*.mvir", &cfg.module_dir().display()))
                .expect("Module directory is not valid.");
            for entry in mdir {
                match entry {
                    Ok(path) => {
                        println_color("Compiling");
                        print!("{:?}\n", &path.display());
                        let m: VerifiedModule = m_runner.complie_module(&path);
                        let cm = &m.as_inner();
                        m_runner.datastore.add_module(&cm.self_id(), cm);
                    }
                    Err(_) => {
                        panic!("Failed to load modules source file.");
                    }
                }
            }

            if !source_path.exists() {
                source_path = cfg.script_dir().join(source_path);
            }

            println_color("Compiling");
            print!("{:?}\n", &source_path.display());
            let compiled_script = m_runner.complie_script(&source_path).into_inner();

            load_genesis(&cfg, &mut m_runner);


            println_color("Running");
            print!(
                "Script: {:?} Args: {:?}\n",
                &source_path.file_name().unwrap(),
                args
            );

            let mut script: Vec<u8> = vec![];
            compiled_script
                .as_inner()
                .serialize(&mut script)
                .expect("Unable to serialize script");

            // Execute script.
            // create a Move VM and populate it with generated modules
            let move_vm = MoveVM::new();
            let mut ctx =
                TransactionExecutionContext::new(GasUnits::new(600), &m_runner.datastore);
            let gas_schedule = gas_schedule::zero_cost_schedule();

            let mut txn_data = TransactionMetadata::default();
            txn_data.sender = cfg.address();

            let result: VMResult<()> =
                move_vm.execute_script(script, &gas_schedule, &mut ctx, &txn_data, vec![], va_tags);

            match result {
                Ok(_) => {
                    let ws = ctx.make_write_set().unwrap();
                    println_color("Output");
                    print!("{} WriteSet was generated\n", &ws.len());

                    for (a, wo) in ws {
                        println!("AccessPath:{}, {:?}", a, wo);
                    }
                    println!("The script runs successfully")
                }
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }
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
