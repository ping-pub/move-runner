use std::{fs, path::PathBuf};

use libra_config::config::{ExecutionConfig, RootPath};
use libra_crypto::{
    ed25519::{Ed25519PrivateKey, Ed25519PublicKey},
    traits::*,
};
use libra_crypto::hash::CryptoHash;
use libra_types::{account_address::AccountAddress, account_address::from_public_key, transaction::Transaction};
use libra_types::transaction::{RawTransaction, SignedTransaction};
use serde::{Deserialize, Serialize};
use stdlib::StdLibOptions;
use vm_genesis;

const DEFAULT_CONFIG_FILE: &str = "Move.toml";

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
        cfg.home = home; // replace home with the value of argument
        cfg
    }

    pub fn initial(&self) {
        fs::create_dir_all(&self.home).expect("Can not create home directory");
        fs::create_dir_all(&self.module_dir()).expect("Failed to create module directory");
        fs::create_dir_all(&self.script_dir()).expect("Failed to create script directory");
        fs::create_dir_all(&self.target_dir()).expect("Failed to create target directory");
        fs::create_dir_all(&self.test_dir()).expect("Failed to create test directory");

        let cfg = toml::to_string_pretty(&self).unwrap();
        fs::write(&self.home.join(DEFAULT_CONFIG_FILE), cfg).expect("Failed to create Move.toml");
    }

    pub fn genesis(&self) {
        let change_set = vm_genesis::generate_genesis_change_set_for_testing(StdLibOptions::Staged);
        let mut cfg = ExecutionConfig::default();

        let priv_key = &Ed25519PrivateKey::from_encoded_string(&self.tx.keypair_private_key).unwrap();
        let raw_txs = RawTransaction::new_change_set(self.address(), self.tx.sequence_number, change_set);
        let signature = priv_key.sign_message(&raw_txs.hash());
        let signed_tx = SignedTransaction::new(raw_txs, self.tx.keypair_public_key.clone(), signature);

        cfg.genesis = Some(Transaction::UserTransaction(signed_tx));
        cfg.save(&RootPath::new(&self.home))
            .expect("genesis.blob was not created");
    }

    pub fn module_dir(&self) -> PathBuf {
        self.home.join(&self.workspace.module_dir)
    }

    pub fn script_dir(&self) -> PathBuf {
        self.home.join(&self.workspace.script_dir)
    }

    pub fn test_dir(&self) -> PathBuf {
        self.home.join(&self.workspace.test_dir)
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
    pub test_dir: PathBuf,
}

impl Default for Workspace {
    fn default() -> Workspace {
        Workspace {
            script_dir: PathBuf::from("src/scripts"),
            module_dir: PathBuf::from("src/modules"),
            target_dir: PathBuf::from("target"),
            test_dir: PathBuf::from("test"),
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
            address: from_public_key(&keypair_public_key),
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
    let private_key = generate_key::generate_key();
    let public_key = private_key.public_key();
    (private_key, public_key)
}

#[test]
fn test_generate_keypair() {
    let (private_key, public_key) = generate_keypair();
    println!(
        "{}=>{}",
        private_key.to_encoded_string().unwrap(),
        public_key
    );
}
