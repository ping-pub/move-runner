use crate::{
    config::Config, 
    commands::Command,
    runner::MoveRunner, 
    Parameter,
};

use bytecode_verifier::verifier::VerifiedModule;
use glob::glob;
use libra_types::transaction::{
    parse_as_transaction_argument,
    TransactionArgument,
};
use move_vm_runtime::MoveVM;
use move_vm_state::{
    //data_cache::{BlockDataCache, RemoteCache},
    execution_context::SystemExecutionContext,
};
use move_vm_types::values::values_impl::Value;
use vm::{
    errors::VMResult,
    gas_schedule::{
        CostTable, GasAlgebra,
        GasUnits},
    transaction_metadata::TransactionMetadata,
};

pub struct RunCommand{}

impl Command for RunCommand{
    fn execute(&self, params: Parameter) {
        if let Parameter::Run{home, source_path, args} = params {

            let ta_args: Vec<TransactionArgument> = args.iter().map(|arg| parse_as_transaction_argument(arg).unwrap()).collect();
            let va_tags = convert_txn_args( &ta_args );

            let u8v = TransactionArgument::U8Vector([1;2].to_vec());

            println!("{:?}", u8v);
                    
            let cfg = Config::load_config(home);
            let mut m_runner = MoveRunner::new(cfg.clone());
            
            println!("loading modules in {}",format!("{}/**/*.mvir", &cfg.module_dir().display()));
            let mdir = glob(&format!("{}/**/*.mvir", &cfg.module_dir().display())).expect("Module directory is not valid.");
            for entry in mdir {
                match entry {
                    Ok(path) => {
                        println!("Compiling: {:?}", &path.display());
                        let m: VerifiedModule = m_runner.complie_module(&path);
                        let cm = &m.as_inner();
                        m_runner.datastore.add_module(&cm.self_id(), cm);
                    },
                    Err(_)=> {
                        panic!("Failed to load modules source file.");
                    }
                }
                
            };

            let compiled_script = m_runner.complie_script(&source_path).into_inner();

            let mut script: Vec<u8> = vec![];
            compiled_script.as_inner()
                .serialize(&mut script)
                .expect("Unable to serialize script"); 
            
            // Execute script. 
            // create a Move VM and populate it with generated modules
            let move_vm = MoveVM::new();
            let mut ctx = SystemExecutionContext::new(&m_runner.datastore, GasUnits::new(0));
            let gas_schedule = CostTable::zero();

            let mut txn_data = TransactionMetadata::default();
            txn_data.sender = cfg.state.address;

            let result: VMResult<()> = move_vm.execute_script(script, &gas_schedule, &mut ctx, &txn_data, vec![], va_tags);
            
            println!("output from move vm: {:?}",  result);
        }
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