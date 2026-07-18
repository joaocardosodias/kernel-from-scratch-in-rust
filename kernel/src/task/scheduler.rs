use alloc::vec::Vec;

use spin::Mutex;

use crate::{
    arch::gdt::TSS,
    task::thread::{Task, TaskState},
};

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

    pub fn block_current_task(&mut self) {
        self.tasks[self.current_index].state = TaskState::Blocked;
    }

    pub fn unblock_task(&mut self, id: usize) {
        for task in &mut self.tasks {
            if task.id == id {
                task.state = TaskState::Ready;
                break;
            }
        }
    }

    pub fn switch_context(&mut self, current_rsp: u64) -> u64 {
        if self.tasks.is_empty() {
            return current_rsp;
        }
        self.tasks[self.current_index].kernel_rsp = current_rsp;

        let mut next_index = self.current_index;
        loop {
            next_index = (next_index + 1) % self.tasks.len();
            if self.tasks[next_index].state == TaskState::Ready {
                break;
            }
            if next_index == self.current_index {
                break;
            }
        }

        self.current_index = next_index;
        let next_task = &self.tasks[self.current_index];
        unsafe {
            let stack_size = 16384;
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
