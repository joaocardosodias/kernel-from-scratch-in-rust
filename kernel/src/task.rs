use alloc::vec::Vec;
use spin::Mutex;
use crate::gdt::TSS;

pub struct Task {
    pub id: usize,
    pub kernel_rsp: u64,
    pub kernel_stack: Vec<u8>,
    pub user_stack: Vec<u8>,
}

impl Task {
    pub fn new(id: usize, entry_point: u64) -> Self {
        let stack_size = 4096;
        let kernel_stack = alloc::vec![0u8; stack_size];
        let user_stack = alloc::vec![0u8; stack_size];

        let kernel_stack_top = kernel_stack.as_ptr() as u64 + stack_size as u64;
        let user_stack_top = user_stack.as_ptr() as u64 + stack_size as u64;

        let mut rsp = kernel_stack_top;

        unsafe {
            // Empilha o Interrupt Frame inicial para o iretq
            rsp -= 8;
            *(rsp as *mut u64) = 0x1B; // SS (User Data)
            rsp -= 8;
            *(rsp as *mut u64) = user_stack_top; // RSP (User Stack)
            rsp -= 8;
            *(rsp as *mut u64) = 0x200; // RFLAGS (Interrupções ativas)
            rsp -= 8;
            *(rsp as *mut u64) = 0x23; // CS (User Code)
            rsp -= 8;
            *(rsp as *mut u64) = entry_point; // RIP (Ponto de entrada)

            // Empilha os 15 registradores gerais iniciados em zero
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
        }
    }
}

pub struct Scheduler {
    pub tasks: Vec<Task>,
    pub current_index: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn switch_context(&mut self, current_rsp: u64) -> u64 {
        if self.tasks.is_empty() {
            return current_rsp;
        }

        // Salva o RSP da stack de Kernel do processo atual
        self.tasks[self.current_index].kernel_rsp = current_rsp;

        // Escolhe o próximo processo (Round-Robin)
        self.current_index = (self.current_index + 1) % self.tasks.len();

        let next_task = &self.tasks[self.current_index];

        // Atualiza a TSS com a stack de Kernel da nova tarefa para as próximas interrupções
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

pub fn start_multitasking() -> ! {
    let first_rsp: u64;
    unsafe {
        let mut sched = SCHEDULER.lock();
        let s = sched.as_mut().unwrap();
        let next_task = &s.tasks[0];
        let stack_size = 4096;
        let top = next_task.kernel_stack.as_ptr() as u64 + stack_size as u64;
        TSS.rsp[0] = top;
        crate::KERNEL_RSP = top;
        first_rsp = next_task.kernel_rsp;
    }

    unsafe {
        core::arch::asm!(
            "mov rsp, {}",
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rbp",
            "pop rdi",
            "pop rsi",
            "pop rdx",
            "pop rcx",
            "pop rbx",
            "pop rax",
            "iretq",
            in(reg) first_rsp,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn timer_interrupt_handler(current_rsp: u64) -> u64 {
    let mut sched = SCHEDULER.lock();
    if let Some(ref mut s) = *sched {
        s.switch_context(current_rsp)
    } else {
        current_rsp
    }
}

core::arch::global_asm!(
    ".global timer_handler_asm",
    "timer_handler_asm:",
    // 1. Salva o contexto dos registradores gerais na pilha de kernel da tarefa atual
    "push rax",
    "push rbx",
    "push rcx",
    "push rdx",
    "push rsi",
    "push rdi",
    "push rbp",
    "push r8",
    "push r9",
    "push r10",
    "push r11",
    "push r12",
    "push r13",
    "push r14",
    "push r15",

    // 2. Avisa o controlador PIC que a interrupção física foi tratada (EOI)
    "mov al, 0x20",
    "out 0x20, al",

    // 3. Passa o RSP atual como argumento (RDI) e chama o escalonador Rust
    "mov rdi, rsp",
    "call timer_interrupt_handler",

    // 4. Carrega o novo RSP (retornado em RAX) para chavear a pilha
    "mov rsp, rax",

    // 5. Restaura os registradores gerais do contexto da nova tarefa
    "pop r15",
    "pop r14",
    "pop r13",
    "pop r12",
    "pop r11",
    "pop r10",
    "pop r9",
    "pop r8",
    "pop rbp",
    "pop rdi",
    "pop rsi",
    "pop rdx",
    "pop rcx",
    "pop rbx",
    "pop rax",

    // 6. Retorna de interrupção (recarrega CS, RIP, RFLAGS, SS, RSP da nova tarefa)
    "iretq"
);
