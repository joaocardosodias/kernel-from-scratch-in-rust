use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Blocked,
}

pub struct Task {
    pub id:           usize,
    pub kernel_rsp:   u64,
    pub kernel_stack: Vec<u8>,
    pub user_stack:   Vec<u8>,
    pub state:        TaskState,
}

impl Task {
    pub fn new(id: usize, entry_point: u64) -> Self {
        let stack_size = 16384;
        let kernel_stack = alloc::vec![0u8; stack_size];
        let user_stack = alloc::vec![0u8; stack_size];

        let kernel_stack_top = kernel_stack.as_ptr() as u64 + stack_size as u64;
        let user_stack_top = user_stack.as_ptr() as u64 + stack_size as u64;

        let mut rsp = kernel_stack_top;

        unsafe {
            rsp -= 8;
            *(rsp as *mut u64) = 0x1B;
            rsp -= 8;
            *(rsp as *mut u64) = user_stack_top;
            rsp -= 8;
            *(rsp as *mut u64) = 0x200;
            rsp -= 8;
            *(rsp as *mut u64) = 0x23;
            rsp -= 8;
            *(rsp as *mut u64) = entry_point;

            for _ in 0..15 {
                rsp -= 8;
                *(rsp as *mut u64) = 0;
            }
        }

        Task {
            id,
            kernel_rsp: rsp,
            kernel_stack,
            user_stack,
            state: TaskState::Ready,
        }
    }
}
