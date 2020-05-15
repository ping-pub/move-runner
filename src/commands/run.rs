use glob::glob;

use bytecode_verifier::verifier::VerifiedModule;
use libra_types::transaction::{
    parse_as_transaction_argument, TransactionArgument
};
use move_core_types::{
    gas_schedule::{ GasAlgebra, GasUnits },
    language_storage::TypeTag
};
use move_vm_runtime::MoveVM;
use move_vm_state::execution_context::{
    ExecutionContext, TransactionExecutionContext
};
use move_vm_types::{
    gas_schedule::zero_cost_schedule,
    transaction_metadata::TransactionMetadata
};
use vm::errors::VMResult;

use crate::{
    commands::{ Command, convert_txn_args, load_genesis, type_parser::parse_type_tags },
    config::Config,
    Parameter,
    println_color,
    runner::MoveRunner
};

pub struct RunCommand {}

impl Command for RunCommand {
    fn execute(&self, params: Parameter) {
        if let Parameter::Run {
            home,
            mut source_path,
            type_args,
            args,
        } = params
        {
            let ty_args: Vec<TypeTag> = parse_type_tags(&type_args.join(",")).unwrap();

            // check if arguments are valid.
            let ta_args: Vec<TransactionArgument> = args
                .iter()
                .map(|arg| parse_as_transaction_argument(arg).unwrap())
                .collect();
            let va_args = convert_txn_args(&ta_args);

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
                "Script: {:?} Type Args:{:?}, Args: {:?}\n",
                &source_path.file_name().unwrap(),
                &ty_args,
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
            let mut ctx = TransactionExecutionContext::new(GasUnits::new(600), &m_runner.datastore);
            let gas_schedule = zero_cost_schedule();

            let mut txn_data = TransactionMetadata::default();
            txn_data.sender = cfg.address();

            let result: VMResult<()> =
                move_vm.execute_script(script, &gas_schedule, &mut ctx, &txn_data, ty_args, va_args);

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
