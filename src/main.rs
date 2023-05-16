#![feature(drain_filter)]

mod process;

use crate::process::Process;
use clap::Parser;
use std::collections::HashMap;
use sysinfo::{Pid, ProcessExt, System, SystemExt};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    sort: bool,
}

fn main() {
    let cli = Cli::parse();
    let sys = System::new_all();

    let mut root = Vec::new();
    let mut processes: HashMap<&Pid, Process> = HashMap::new();

    for (pid, process) in sys.processes() {
        match process
            .parent()
            .and_then(|parent| processes.get_mut(&parent))
        {
            None => root.push(pid),
            Some(parent) => parent.children.get_or_insert_with(Vec::new).push(pid),
        }

        processes.insert(pid, Process::new(process));
    }

    root.drain_filter(|pid| {
        match processes[pid]
            .parent()
            .and_then(|parent| processes.get_mut(&parent))
        {
            None => false,
            Some(parent) => {
                parent.children.get_or_insert_with(Vec::new).push(pid);
                true
            }
        }
    });

    if cli.sort {
        root.sort_unstable();
    }
}
