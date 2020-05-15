use std::{ fs, io::Write, path::{ Path, PathBuf } };

use bytecode_verifier::verifier::{ VerifiedModule, VerifiedScript };
use compiler::Compiler;
use language_e2e_tests::data_store::FakeDataStore;
use stdlib::{ stdlib_modules, StdLibOptions };

use crate::{ config::Config, println_color };

pub struct MoveRunner {
    cfg: Config,
    stdlib: Vec<VerifiedModule>,
    pub datastore: FakeDataStore,
}

impl MoveRunner {
    pub fn new(cfg: Config) -> Self {
        println_color("Loaded");
        print!(" Current address: 0x{:?}\n", cfg.address());
        MoveRunner {
            cfg,
            stdlib: stdlib_modules(StdLibOptions::Staged).to_vec(),
            datastore: FakeDataStore::default(),
        }
    }

    pub fn complie_module(&mut self, path: &Path) -> VerifiedModule {
        let c = Compiler {
            address: self.cfg.address(),
            skip_stdlib_deps: false,
            extra_deps: self.stdlib.clone(),
            ..Compiler::default()
        };

        let source = fs::read_to_string(path).expect("Failed to load source file:");

        let compiled_module = c
            .into_compiled_module(path.as_os_str().to_str().unwrap(), &source)
            .expect("Failed to compile module");
        if self.cfg.compile.output_move_bytecode {
            let mut bytes: Vec<u8> = vec![];
            compiled_module
                .as_inner()
                .serialize(&mut bytes)
                .expect("Unable to serialize module");

            //PathBuf::from(path.file_name().)
            let outpath = self
                .cfg
                .target_dir()
                .join(PathBuf::from(path.file_name().unwrap()))
                .with_extension("mv");
            write_output(&outpath, &bytes);
        }

        let verified_module = VerifiedModule::new(compiled_module).unwrap();
        self.stdlib.push(verified_module.clone()); // add module to stdlib
        verified_module
    }

    pub fn complie_script(&self, path: &Path) -> VerifiedScript {
        let c = Compiler {
            address: self.cfg.address(),
            skip_stdlib_deps: false,
            extra_deps: self.stdlib.clone(),
            ..Compiler::default()
        };

        let source = fs::read_to_string(path).expect("Failed to load source file:");

        let (compiled_script, source_map) = c
            .into_compiled_script_and_source_map(path.as_os_str().to_str().unwrap(), &source)
            .expect("Failed to compile module");

        if self.cfg.compile.output_source_map {
            let bytes = serde_json::to_vec(&source_map).expect("Unable to serialize script");

            let outpath = self
                .cfg
                .target_dir()
                .join(PathBuf::from(path.file_name().unwrap()))
                .with_extension("mvsm");
            write_output(&outpath, &bytes);
        }

        if self.cfg.compile.output_move_bytecode {
            let mut bytes: Vec<u8> = vec![];
            compiled_script
                .as_inner()
                .serialize(&mut bytes)
                .expect("Unable to serialize script");

            let outpath = self
                .cfg
                .target_dir()
                .join(PathBuf::from(path.file_name().unwrap()))
                .with_extension("mv");
            write_output(&outpath, &bytes);
        }

        VerifiedScript::new(compiled_script).unwrap()
    }
}

fn write_output(path: &PathBuf, buf: &[u8]) {
    let mut f = fs::File::create(path).expect("Error occurs on create output file");
    f.write_all(&buf).expect("Error occurs on writing output file");
}
