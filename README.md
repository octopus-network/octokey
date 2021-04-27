# octokey

octokey is a tool for handling substrate keys.

The files in the keys_backup are backups for octopus team.

The files in the keys_octoup are necessary to run octoup.

The file in the keys_chainspec is used to update chainspec.json

```
$ cargo run
$ tree keys/
keys/
├── keys_backup
│   ├── 0
│   │   ├── babe
│   │   ├── gran
│   │   ├── imon
│   │   ├── node-key
│   │   ├── octo
│   │   ├── peer-id
│   │   └── validator
│   ├── 1
│   │   ├── babe
│   │   ├── gran
│   │   ├── imon
│   │   ├── node-key
│   │   ├── octo
│   │   ├── peer-id
│   │   └── validator
│   ├── 2
│   │   ├── babe
│   │   ├── gran
│   │   ├── imon
│   │   ├── node-key
│   │   ├── octo
│   │   ├── peer-id
│   │   └── validator
│   └── 3
│       ├── babe
│       ├── gran
│       ├── imon
│       ├── node-key
│       ├── octo
│       ├── peer-id
│       └── validator
├── keys_chainspec
│   └── chainspec.json
└── keys_octoup
    ├── 0
    │   ├── babe.json
    │   ├── gran.json
    │   ├── imon.json
    │   ├── node-key
    │   ├── octo.json
    │   └── peer-id
    ├── 1
    │   ├── babe.json
    │   ├── gran.json
    │   ├── imon.json
    │   ├── node-key
    │   ├── octo.json
    │   └── peer-id
    ├── 2
    │   ├── babe.json
    │   ├── gran.json
    │   ├── imon.json
    │   ├── node-key
    │   ├── octo.json
    │   └── peer-id
    └── 3
        ├── babe.json
        ├── gran.json
        ├── imon.json
        ├── node-key
        ├── octo.json
        └── peer-id

11 directories, 53 files
```
