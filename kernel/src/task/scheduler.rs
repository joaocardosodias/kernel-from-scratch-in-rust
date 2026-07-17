use alloc::vec::Vec;

use spin::Mutex;

use crate::{arch::gdt::TSS, task::thread::Task};

pub struct Scheduler {
    pub tasks:         Vec<Task>,
    pub current_index: usize,
}

impl Default for Scheduler {
    fn default() -> Self { Self::new() }
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks:         Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_task(&mut self, task: Task) { self.tasks.push(task); }

    pub fn switch_context(&mut self, current_rsp: u64) -> u64 {
        if self.tasks.is_empty() {
            return current_rsp;
        }
        self.tasks[self.current_index].kernel_rsp = current_rsp;
        self.current_index = (self.current_index + 1) % self.tasks.len();
        let next_task = &self.tasks[self.current_index];
        unsafe {
            let stack_size = 4096;
            let top = next_task.kernel_stack.as_ptr() as u64 + stack_size as u64;
            TSS.rsp[0] = top;
            crate::KERNEL_RSP = top;
        }
        next_task.kernel_rsp
    }
}

pub static SCHEDULER: Mutex<Option<Scheduler>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn timer_interrupt_handler(current_rsp: u64) -> u64 {
    let mut sched = SCHEDULER.lock();
    if let Some(ref mut s) = *sched {
        s.switch_context(current_rsp)
    } else {
        current_rsp
    }
}
