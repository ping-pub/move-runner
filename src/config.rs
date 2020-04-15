use std::{fs, path::PathBuf};

use libra_crypto::{ed25519::{Ed25519PrivateKey, Ed25519PublicKey}, traits::*, Uniform};
use libra_types::account_address::AccountAddress;
use rand::{
    Rng,
    rngs::{OsRng, StdRng}, SeedableRng,
};
use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG_FILE: &str = "Move.toml";
const GENESIS_BLOB: &str = "genesis.blob";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    project_name: String,
    pub home: PathBuf,
    pub workspace: Workspace,
    pub compile: Compile,
    pub tx: DevTransaction,
    pub storage: Storage,
}

impl Config {
    pub fn new(name: String, home: PathBuf) -> Self {
        Self {
            project_name: name,
            home,
            workspace: Workspace::default(),
            compile: Compile::default(),
            tx: DevTransaction::default(),
            storage: Storage::default(),
        }
    }

    pub fn load_config(home: PathBuf) -> Self {
        crate::println_color("Loading");
        print!("config from {:?}\n", &home.join(DEFAULT_CONFIG_FILE));
        let content = fs::read_to_string(&home.join(DEFAULT_CONFIG_FILE))
            .expect("Failed to loaded config files");
        let mut cfg: Self = toml::from_str(&content).expect("Failed to loaded Move.toml");
        cfg.home = home;  // replace home with the value of argument
        cfg
    }

    pub fn initial(&self) {
        fs::create_dir_all(&self.home).expect("Can not create home directory");
        fs::create_dir_all(&self.module_dir()).expect("Failed to create module directory");
        fs::create_dir_all(&self.script_dir()).expect("Failed to create script directory");
        fs::create_dir_all(&self.target_dir()).expect("Failed to create target directory");

        let cfg = toml::to_string_pretty(&self).unwrap();
        fs::write(&self.home.join(DEFAULT_CONFIG_FILE), cfg).expect("Failed to create Move.toml");
    }

    pub fn genesis(&self) {
        fs::write(&self.home.join(GENESIS_BLOB), []).expect("Failed to create genesis.blob");
    }

    pub fn module_dir(&self) -> PathBuf {
        self.home.join(&self.workspace.module_dir)
    }

    pub fn script_dir(&self) -> PathBuf {
        self.home.join(&self.workspace.script_dir)
    }

    pub fn target_dir(&self) -> PathBuf {
        self.home.join(&self.workspace.target_dir)
    }

    pub fn address(&self) -> AccountAddress {
        self.tx.address
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
pub struct DevTransaction {
    pub address: AccountAddress,
    pub keypair_private_key: String,
    pub keypair_public_key: Ed25519PublicKey,
    pub sequence_number: u64,
}

impl Default for DevTransaction {
    fn default() -> Self {
        let (private_key, keypair_public_key) = generate_keypair();
        Self {
            address: AccountAddress::from_public_key(&keypair_public_key),
            sequence_number: 0,
            keypair_private_key: private_key.to_encoded_string().unwrap(),
            keypair_public_key,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Storage {
    pub load_state_from_genesis: bool,
    pub save_writeset_to_genesis: bool,
}

/// Generate an Ed25519 key pair.
fn generate_keypair() -> (Ed25519PrivateKey, Ed25519PublicKey) {
    let mut seed_rng = OsRng::new().expect("can't access OsRng");
    let seed: [u8; 32] = seed_rng.gen();
    let mut stdrng = StdRng::from_seed(seed);
    let private_key = Ed25519PrivateKey::generate(&mut stdrng);
    let public_key = private_key.public_key();
    (private_key, public_key)
}

#[test]
fn test_generate_keypair() {
    let (private_key, public_key) = generate_keypair();
    println!("{}=>{}", private_key.to_encoded_string().unwrap(), public_key);
}


