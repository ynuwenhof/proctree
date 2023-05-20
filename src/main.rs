mod process;

use crate::process::Process;
use clap::Parser;
use std::collections::HashMap;
use sysinfo::{Pid, ProcessExt, System, SystemExt};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, env = "PROCTREE_UNSORTED")]
    unsorted: bool,
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

    // TODO: https://github.com/rust-lang/rust/issues/43244
    // TODO: Replace this implementation once the `drain_filter` feature has been stabilized
    let mut i = 0;
    while i < root.len() {
        let pid = root[i];
        match processes[pid]
            .parent()
            .and_then(|parent| processes.get_mut(&parent))
        {
            None => i += 1,
            Some(parent) => {
                parent.children.get_or_insert_with(Vec::new).push(pid);
                root.swap_remove(i);
            }
        }
    }

    if !cli.unsorted {
        root.sort_unstable();
    }

    let mut buf = String::new();
    let len = root.len().saturating_sub(1);

    for (i, pid) in root.iter().enumerate() {
        let process = &processes[pid];
        print_process(&processes, process, &mut buf, i == len);
    }
}

fn print_process(
    processes: &HashMap<&Pid, Process>,
    process: &Process,
    buf: &mut String,
    last: bool,
) {
    let (symbol, branch) = if last { (' ', '└') } else { ('│', '├') };
    println!("{buf}{branch}──{} {}", process.pid(), process.name());

    buf.push(symbol);
    buf.push_str("  ");

    if let Some(children) = &process.children {
        let len = children.len().saturating_sub(1);
        for (i, pid) in children.iter().enumerate() {
            let child = &processes[pid];
            print_process(processes, child, buf, i == len)
        }
    }

    buf.truncate(buf.len() - 2 - symbol.len_utf8());
}
