use std::fs;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// The name of appchain
    #[structopt(short, long)]
    appchain: String,

    /// Number of validators
    #[structopt(short, long, default_value = "4")]
    number: u32,

    /// Output dir
    #[structopt(short, long, default_value = ".", parse(from_os_str))]
    output: PathBuf,
}

#[derive(Debug, Default)]
struct Key {
    secret_seed: String,
    public_key: String,
    address: String,
}

#[derive(Debug, Default)]
struct Validator {
    id: Key,
    aura: Key,
    gran: Key,
    octo: Key,
    peer_id: String,
}

use serde_derive::Serialize;

#[derive(Serialize)]
struct Config {
    ip: String,
    port: Option<u16>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);

    let output = Command::new("subkey")
        .arg("-V")
        .output()
        .expect("failed to execute process");
    let s = match std::str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("{} detected.", s.trim());

    let base_path = opt.output.join(opt.appchain);
    for i in 0..opt.number {
        let backup_path = base_path.join("keys_backup").join(format!("{}", i));
        let octoup_path = base_path.join("keys_octoup").join(format!("{}", i));

        fs::create_dir_all(&backup_path).unwrap();
        fs::create_dir_all(&octoup_path).unwrap();

        let mut val = Validator::default();
        val.id = generate_key(&backup_path, &octoup_path, "validator", "sr25519");
        val.aura = generate_key(&backup_path, &octoup_path, "aura", "sr25519");
        val.gran = generate_key(&backup_path, &octoup_path, "gran", "ed25519");
        val.octo = generate_key(&backup_path, &octoup_path, "octo", "sr25519");

        let output = Command::new("subkey")
            .arg("generate-node-key")
            .arg("--file")
            .arg(backup_path.join("node-key"))
            .output()
            .expect("failed to execute process");
        fs::write(backup_path.join("peer-id"), output.stderr.clone()).unwrap();
        fs::copy(backup_path.join("node-key"), octoup_path.join("node-key")).unwrap();
        fs::copy(backup_path.join("peer-id"), octoup_path.join("peer-id")).unwrap();

        let s = match std::str::from_utf8(&output.stderr) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        val.peer_id = s.trim().to_string();
        println!("validator-{}: {:#?}", i, val);
    }

    let config = Config {
        ip: "127.0.0.1".to_string(),
        port: None,
    };

    let toml = toml::to_string(&config).unwrap();
    let chainspec_path = base_path.join("keys_chainspec");
    fs::create_dir_all(&chainspec_path).unwrap();
    fs::write(chainspec_path.join("chainspec.toml"), toml).unwrap();
}

fn generate_key(backup_path: &PathBuf, octoup_path: &PathBuf, typ: &str, scheme: &str) -> Key {
    let output = Command::new("subkey")
        .arg("generate")
        .arg("--scheme")
        .arg(scheme)
        .output()
        .expect("failed to execute process");
    fs::write(backup_path.join(typ), output.stdout.clone()).unwrap();

    let s = match std::str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let lines = s.trim().lines();
    let mut key = Key::default();
    for line in lines {
        match line.trim() {
            line if line.starts_with("Secret seed:") => {
                key.secret_seed = line.split(":").collect::<Vec<&str>>()[1].trim().to_string();
            }
            line if line.starts_with("Public key (hex):") => {
                key.public_key = line.split(":").collect::<Vec<&str>>()[1].trim().to_string();
            }
            line if line.starts_with("SS58 Address:") => {
                key.address = line.split(":").collect::<Vec<&str>>()[1].trim().to_string();
            }
            _ => {}
        }
    }
    if typ != "validator" {
        let json = format!(
            r#"{{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_insertKey",
  "params": [
    "{}",
    "{}",
    "{}"
  ]
}}"#,
            typ, key.secret_seed, key.public_key
        );
        fs::write(octoup_path.join(format!("{}.json", typ)), json).unwrap();
    }
    key
}
