# octokey

octokey is a tool for handling substrate keys.

The files in the keys_backup are backups for octopus team.

The file **appchain-launch.hcl** is used to deploy bootnodes by the terraform cloud.

The file **chainspec-snippet.json** is used to update chainspec.json.

```
$ cargo run -- --appchain-id <AppchainID> --testnet
$ tree keys/
keys/
├── appchain-launch.hcl
├── chainspec-snippet.json
└── keys_backup
    ├── 0
    │   ├── babe
    │   ├── babe.json
    │   ├── beef
    │   ├── beef.json
    │   ├── gran
    │   ├── gran.json
    │   ├── imon
    │   ├── imon.json
    │   ├── node-key
    │   ├── octo
    │   ├── octo.json
    │   ├── peer-id
    │   └── validator
    ├── 1
    │   ├── babe
    │   ├── babe.json
    │   ├── beef
    │   ├── beef.json
    │   ├── gran
    │   ├── gran.json
    │   ├── imon
    │   ├── imon.json
    │   ├── node-key
    │   ├── octo
    │   ├── octo.json
    │   ├── peer-id
    │   └── validator
    ├── 2
    │   ├── babe
    │   ├── babe.json
    │   ├── beef
    │   ├── beef.json
    │   ├── gran
    │   ├── gran.json
    │   ├── imon
    │   ├── imon.json
    │   ├── node-key
    │   ├── octo
    │   ├── octo.json
    │   ├── peer-id
    │   └── validator
    └── 3
        ├── babe
        ├── babe.json
        ├── beef
        ├── beef.json
        ├── gran
        ├── gran.json
        ├── imon
        ├── imon.json
        ├── node-key
        ├── octo
        ├── octo.json
        ├── peer-id
        └── validator

5 directories, 54 files
```
