# Proctree

Show running processes in a tree structure.

```
> proctree
├──1 systemd
|  ├──1028 upowerd
|  ├──1040 rtkit-daemon
|  ├──1095 ModemManagere
|  ├──840 NetworkManager
|  ├──527 systemd-oomd
|  ├──885 cupsd
|  └──876 gdm
|     └──1275 gdm-session-wor
|        └──1312 gdm-wayland-ses
|           └──1315 .gnome-session-
└──2 kthreadd
   ├──2982 kworker/3:3-events
   ├──27 kworker/2:0H-events_highpri
   ├──12 rcu_tasks_rude_
   ├──22 kworker/1:0H-kblockd
   ├──33 kdevtmpfs
   ├──34 inet_frag_wq
   └──61 blkcg_punt_bio
```

## Installation

### Cargo

Make sure the current stable release of [Rust](https://rust-lang.org/tools/install) is installed.

#### Registry

```bash
cargo install proctree
```

#### Manual

```bash
git clone https://github.com/ynuwenhof/proctree.git
cd proctree
cargo install --path .
```

After installing, you can run the application with:

```bash
proctree --unsorted
```

this will print the process tree into the terminal.

## Configuration

Proctree can be configured via environment variables or command line arguments.

Missing keys will fallback to their default value.

| Key                   | Description                                   | Default |
|-----------------------|-----------------------------------------------|---------|
| `PROCTREE_UNSORTED`   | Print the process tree without sorting by pid | `false` |

## License

This project is licensed under either of the following licenses, at your option:

* [Apache License, Version 2.0](https://apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ynuwenhof/proctree/blob/main/LICENSE-APACHE))
* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ynuwenhof/proctree/blob/main/LICENSE-MIT))
