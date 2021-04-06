# octokey

octokey is a tool for handling substrate keys.

The files in the keys_backup are backups for octopus team.
The files in the keys_octoup are necessary to run octoup.
The file in the keys_chainspec is used to update chainspec.json

```
$ cargo run -- --appchain barnacle
$ tree barnacle
barnacle
├── keys_backup
│   ├── 0
│   │   ├── aura
│   │   ├── gran
│   │   ├── node-key
│   │   ├── octo
│   │   ├── peer-id
│   │   └── validator
│   ├── 1
│   │   ├── aura
│   │   ├── gran
│   │   ├── node-key
│   │   ├── octo
│   │   ├── peer-id
│   │   └── validator
│   ├── 2
│   │   ├── aura
│   │   ├── gran
│   │   ├── node-key
│   │   ├── octo
│   │   ├── peer-id
│   │   └── validator
│   └── 3
│       ├── aura
│       ├── gran
│       ├── node-key
│       ├── octo
│       ├── peer-id
│       └── validator
├── keys_chainspec
│   └── chainspec.toml
└── keys_octoup
    ├── 0
    │   ├── aura.json
    │   ├── gran.json
    │   ├── node-key
    │   ├── octo.json
    │   └── peer-id
    ├── 1
    │   ├── aura.json
    │   ├── gran.json
    │   ├── node-key
    │   ├── octo.json
    │   └── peer-id
    ├── 2
    │   ├── aura.json
    │   ├── gran.json
    │   ├── node-key
    │   ├── octo.json
    │   └── peer-id
    └── 3
        ├── aura.json
        ├── gran.json
        ├── node-key
        ├── octo.json
        └── peer-id

11 directories, 45 files
```
