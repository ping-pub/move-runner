use bytecode_verifier::verifier::VerifiedModule;
use move_core_types::{
    gas_schedule::{GasAlgebra, GasUnits},
};
use move_vm_runtime::MoveVM;
use move_vm_state::execution_context::{ExecutionContext, TransactionExecutionContext};
use vm::{
    errors::VMResult,
    gas_schedule,
    transaction_metadata::TransactionMetadata,
};

use glob::glob;

use crate::{commands::Command, config::Config, Parameter, println_color, runner::MoveRunner};
use crate::commands::load_genesis;

pub struct TestCommand {}

impl Command for TestCommand {
    fn execute(&self, params: Parameter) {
        if let Parameter::Test {
            home,
        } = params
        {
            // initialize
            let cfg = Config::load_config(home);
            let mut m_runner = MoveRunner::new(cfg.clone());
            load_genesis(&cfg, &mut m_runner);

            // loading dependencies
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

            // loading test cases
            println_color("Loading");
            print!("test cases from {}\n", &cfg.test_dir().display());
            let mdir = glob(&format!("{}/**/*.mvir", &cfg.test_dir().display()))
                .expect("Module directory is not valid.");
            for entry in mdir {
                match entry {
                    Ok(path) => {
                        println_color("Compiling");
                        print!("{:?}\n", &path);
                        let compiled_script = m_runner.complie_script(&path).into_inner();

                        println_color("Running");
                        print!(
                            "Script: {:?} Args: []\n",
                            &path.file_name().unwrap()
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
                            move_vm.execute_script(script, &gas_schedule, &mut ctx, &txn_data, vec![], vec![]);

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
                    },
                    Err(_) => {
                        panic!("Failed to load source file of test cases.");
                    }
                }
            } // for
        }
    }
}

