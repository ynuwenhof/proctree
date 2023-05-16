use sysinfo::Pid;

pub struct Process<'a> {
    process: &'a sysinfo::Process,
    pub children: Option<Vec<&'a Pid>>,
}

impl<'a> Process<'a> {
    pub fn new(process: &'a sysinfo::Process) -> Self {
        Self {
            process,
            children: None,
        }
    }
}
