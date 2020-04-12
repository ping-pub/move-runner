use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
//use libra_crypto::{ed25519::Ed25519PrivateKey, test_utils::TEST_SEED, PrivateKey, Uniform};
use libra_types::{ account_address::AccountAddress };

const DEFAULT_CONFIG_FILE: &str = "Move.toml";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    project_name: String,
    home: PathBuf,
    pub workspace: Workspace,
    pub compile: Compile,
    pub state: DevState,
}

impl Config {

    pub fn new(name: String, home: PathBuf) -> Self {
        Self {
            project_name: name,
            home,
            workspace: Workspace::default(),
            compile: Compile::default(),
            state: DevState::default(),
        }
    }

    pub fn load_config(home : PathBuf) -> Self {
        println!("loaded config from {:?}", &home.join(DEFAULT_CONFIG_FILE));
        let content = fs::read_to_string(&home.join( DEFAULT_CONFIG_FILE )).expect("Failed to loaded config files");
        toml::from_str(&content).expect("Failed to loaded Move.toml")
    }

    pub fn initial(&self) {
        
        fs::create_dir_all(&self.home).expect("Can not create home directory");
        fs::create_dir_all(&self.module_dir()).expect("Failed to create module directory");
        fs::create_dir_all(&self.script_dir()).expect("Failed to create script directory");
        fs::create_dir_all(&self.target_dir()).expect("Failed to create target directory");

        let cfg = toml::to_string_pretty(&self).unwrap();
        fs::write(&self.home.join(DEFAULT_CONFIG_FILE), cfg).expect("Failed to create Move.toml");
    }

    pub fn module_dir(&self)-> PathBuf {
        self.home.join(&self.workspace.module_dir)
    }

    pub fn script_dir(&self)-> PathBuf {
        self.home.join(&self.workspace.script_dir)
    }

    pub fn target_dir(&self)-> PathBuf {
        self.home.join(&self.workspace.target_dir)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Workspace {
    /// Script path
    pub script_dir: PathBuf,
    pub module_dir: PathBuf,
    pub target_dir: PathBuf,
}

impl Default for Workspace {
    fn default() -> Workspace {
        Workspace {
            script_dir: PathBuf::from("src/scripts"),
            module_dir: PathBuf::from("src/modules"),
            target_dir: PathBuf::from("target"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Compile {
    /// Script path
    pub output_source_map: bool,
    pub output_move_bytecode: bool,
    pub skip_stdlib: bool,
    pub custom_stdlib: bool,
    pub custom_stdlib_path: PathBuf,
}

impl Default for Compile {
    fn default() -> Compile {
        Compile {
            output_source_map: true,
            output_move_bytecode: true,
            skip_stdlib: false,
            custom_stdlib: false,
            custom_stdlib_path: PathBuf::from("src/stdlib"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevState {
    /// Script path
    pub address: AccountAddress,
    // pub private_key: String,
    // pub public_key: String,
}

impl Default for DevState {
    fn default() -> DevState {
        DevState {
            address: AccountAddress::random(),
            // private_key: ,
            // public_key: public_key.to_string(),
        }
    }
}
