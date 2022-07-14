use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;
use tinytemplate::TinyTemplate;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChainSpec {
	boot_nodes: Vec<String>,
	genesis: Genesis,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Genesis {
	runtime: Runtime,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Runtime {
	balances: Balances,
	octopus_appchain: OctopusAppchain,
	session: Session,
	sudo: Sudo,
	anchor_contract: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Balances {
	balances: Vec<(String, u128)>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct SessionKeys {
	babe: String,
	grandpa: String,
	im_online: String,
	beefy: String,
	octopus: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Session {
	keys: Vec<(String, String, SessionKeys)>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Sudo {
	key: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct OctopusAppchain {
	validators: Vec<(String, u128)>,
}

#[derive(Debug, Default, Serialize)]
struct Context {
	nodes: Vec<Node>,
	keys: Vec<Keys>,
}

static TEMPLATE: &'static str = r#"
keys_octoup_node = [
{{ - for node in nodes }}
        \{
            node-key = "{ node.node_key }",
            peer-id = "{ node.peer_id }"
        }{{ if not @last }},{{ endif }}
{{ - endfor }}
]

keys_octoup_session = [
{{ - for key in keys}}
        \{
            "babe.json" = "\{\n  \"jsonrpc\":\"2.0\",\n  \"id\":1,\n  \"method\":\"author_insertKey\",\n  \"params\": [\n    \"babe\",\n    \"{ key.babe.secret_seed }\",\n    \"{ key.babe.public_key }\"\n  ]\n}",
            "beef.json" = "\{\n  \"jsonrpc\":\"2.0\",\n  \"id\":1,\n  \"method\":\"author_insertKey\",\n  \"params\": [\n    \"beef\",\n    \"{ key.beef.secret_seed }\",\n    \"{ key.beef.public_key }\"\n  ]\n}",
            "gran.json" = "\{\n  \"jsonrpc\":\"2.0\",\n  \"id\":1,\n  \"method\":\"author_insertKey\",\n  \"params\": [\n    \"gran\",\n    \"{ key.gran.secret_seed }\",\n    \"{ key.gran.public_key }\"\n  ]\n}",
            "imon.json" = "\{\n  \"jsonrpc\":\"2.0\",\n  \"id\":1,\n  \"method\":\"author_insertKey\",\n  \"params\": [\n    \"imon\",\n    \"{ key.imon.secret_seed }\",\n    \"{ key.imon.public_key }\"\n  ]\n}",
            "octo.json" = "\{\n  \"jsonrpc\":\"2.0\",\n  \"id\":1,\n  \"method\":\"author_insertKey\",\n  \"params\": [\n    \"octo\",\n    \"{ key.octo.secret_seed }\",\n    \"{ key.octo.public_key }\"\n  ]\n}"
        }{{ if not @last }},{{ endif }}
{{ - endfor }}
]
"#;

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

	/// Appchain id
	#[structopt(short, long)]
	appchain_id: String,

	/// For testnet
	#[structopt(short, long)]
	testnet: bool,

	/// Number of validators
	#[structopt(short, long, default_value = "4")]
	number: u32,

	/// Output dir
	#[structopt(short, long, default_value = ".", parse(from_os_str))]
	output: PathBuf,
}

#[derive(Debug, Default, Clone, Serialize)]
struct Key {
	secret_seed: String,
	public_key: String,
	address: String,
}

#[derive(Debug, Default, Clone, Serialize)]
struct Keys {
	babe: Key,
	gran: Key,
	imon: Key,
	beef: Key,
	octo: Key,
}

#[derive(Debug, Default, Serialize)]
struct Node {
	node_key: String,
	peer_id: String,
}

fn main() {
	let opt = Opt::from_args();
	println!("{:#?}", opt);
	if opt.number < 1 {
		println!("The number of validators should be greater than 1.");
		return;
	}

	let output = Command::new("subkey").arg("-V").output().expect("command subkey not found");
	let s = match std::str::from_utf8(&output.stdout) {
		Ok(v) => v,
		Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
	};
	println!("{} detected.", s.trim());
	let mut tt = TinyTemplate::new();
	tt.add_template("launch", TEMPLATE).unwrap();
	let mut context = Context::default();

	let mut chainspec = ChainSpec::default();
	let base_path = opt.output.join("keys");
	for i in 0..opt.number {
		let backup_path = base_path.join("keys_backup").join(format!("{}", i));

		fs::create_dir_all(&backup_path).unwrap();

		let id = generate_key(&backup_path, "validator", "sr25519");
		let babe = generate_key(&backup_path, "babe", "sr25519");
		let gran = generate_key(&backup_path, "gran", "ed25519");
		let imon = generate_key(&backup_path, "imon", "sr25519");
		let beef = generate_key(&backup_path, "beef", "ecdsa");
		let octo = generate_key(&backup_path, "octo", "sr25519");
		context.keys.push(Keys {
			babe: babe.clone(),
			gran: gran.clone(),
			imon: imon.clone(),
			beef: beef.clone(),
			octo: octo.clone(),
		});

		let output = Command::new("subkey")
			.arg("generate-node-key")
			.arg("--file")
			.arg(backup_path.join("node-key"))
			.output()
			.expect("failed to execute process");
		fs::write(backup_path.join("peer-id"), output.stderr.clone()).unwrap();
		let node_key = fs::read_to_string(backup_path.join("node-key"))
			.expect("Something went wrong reading node-key");

		let s = match std::str::from_utf8(&output.stderr) {
			Ok(v) => v,
			Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
		};
		let peer_id = s.trim().to_string();
		context.nodes.push(Node { node_key, peer_id: peer_id.clone() });
		let mut anchor_contract = opt.appchain_id.clone();
		if opt.testnet {
			anchor_contract.push_str(".registry.test_oct.testnet");
		} else {
			anchor_contract.push_str(".octopus-registry.near");
		}
		chainspec.boot_nodes.push(format!(
			"/dns/bootnode-v1-{}.{}.{}.octopus.network/tcp/30333/ws/p2p/{}",
			i,
			opt.appchain_id,
			if opt.testnet { "testnet" } else { "mainnet" },
			peer_id
		));
		if i == 0 {
			chainspec
				.genesis
				.runtime
				.balances
				.balances
				.push((id.address.clone(), 510_000_000_000_000_000_000));
			chainspec.genesis.runtime.sudo = Sudo { key: id.address.clone() };
			chainspec.genesis.runtime.anchor_contract = anchor_contract;
		} else {
			chainspec
				.genesis
				.runtime
				.balances
				.balances
				.push((id.address.clone(), 10_000_000_000_000_000_000));
		}
		let session_keys = SessionKeys {
			babe: babe.address,
			grandpa: gran.address,
			im_online: imon.address,
			beefy: beef.address,
			octopus: octo.address,
		};
		chainspec.genesis.runtime.session.keys.push((
			id.address.clone(),
			id.address.clone(),
			session_keys,
		));
		chainspec
			.genesis
			.runtime
			.octopus_appchain
			.validators
			.push((id.address, 10_000_000_000_000_000_000_000));
	}

	let json = serde_json::to_string_pretty(&chainspec).unwrap();
	fs::write(base_path.join("chainspec-snippet.json"), json).unwrap();

	let rendered = tt.render("launch", &context).unwrap();
	println!("{}", rendered);
	fs::write(base_path.join("appchain-launch.hcl"), rendered).unwrap();
}

fn generate_key(backup_path: &PathBuf, typ: &str, scheme: &str) -> Key {
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
			},
			line if line.starts_with("Public key (hex):") => {
				key.public_key = line.split(":").collect::<Vec<&str>>()[1].trim().to_string();
			},
			line if line.starts_with("Public key (SS58):") => {
				key.address = line.split(":").collect::<Vec<&str>>()[1].trim().to_string();
			},
			_ => {},
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
		fs::write(backup_path.join(format!("{}.json", typ)), json).unwrap();
	}
	key
}
