[bits 16]
[org 0x7E00]

_start:
    cli
    mov [drive_num], dl
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7E00
    sti
    mov ax, 0x2401
    int 0x15

    mov word [dap + 2], 127
    mov dword [dap + 8], 3
    mov dword [dap + 12], 0
    mov word [dap + 4], 0
    mov word [dap + 6], 0x2000
    mov cx, 7
.load_loop:
    push cx
    mov si, dap
    mov ah, 0x42
    mov dl, [drive_num]
    int 0x13
    jc vbe_fail
    add dword [dap + 8], 127
    add word [dap + 6], 0xFE0
    pop cx
    loop .load_loop

    xor ax, ax
    mov es, ax
    mov di, 0x4000
    mov ax, 0x4F00
    int 0x10
    cmp ax, 0x004F
    jne vbe_fail
    mov si, [0x400E]
    mov ax, [0x4010]
    mov fs, ax

search_mode:
    mov cx, [fs:si]
    cmp cx, 0xFFFF
    je vbe_fail
    push si
    push cx
    xor ax, ax
    mov es, ax
    mov di, 0x5000
    mov ax, 0x4F01
    int 0x10
    pop cx
    pop si
    cmp ax, 0x004F
    jne next_mode
    mov ax, [0x5012]
    cmp ax, 1920
    jne next_mode
    mov ax, [0x5014]
    cmp ax, 1080
    jne next_mode
    mov al, [0x5019]
    cmp al, 32
    jne next_mode
    mov ax, [0x5000]
    test ax, 0x80
    jz next_mode
    jmp found_mode

next_mode:
    add si, 2
    jmp search_mode

found_mode:
    mov eax, [0x5028]
    mov [0x7000], eax
    mov dword [0x7004], 0
    movzx eax, word [0x5012]
    mov [0x7008], eax
    movzx eax, word [0x5014]
    mov [0x700C], eax
    movzx eax, word [0x5010]
    mov [0x7010], eax
    or cx, 0x4000
    mov ax, 0x4F02
    mov bx, cx
    int 0x10
    cmp ax, 0x004F
    jne vbe_fail
    lgdt [gdt_ptr]
    cli
    mov eax, cr0
    or  eax, 1
    mov cr0, eax
    jmp 0x08:protected_mode

vbe_fail:
    cli
    hlt
    jmp vbe_fail

drive_num: db 0

align 4
dap:
    db 0x10
    db 0
    dw 0
    dw 0x0000
    dw 0x2000
    dq 3

gdt_start:
    dq 0

gdt_code32:
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 10011010b
    db 11001111b
    db 0x00

gdt_data:
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 10010010b
    db 11001111b
    db 0x00

gdt_code64:
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 10011010b
    db 10101111b
    db 0x00

gdt_end:

gdt_ptr:
    dw gdt_end - gdt_start - 1
    dd gdt_start

[bits 32]

protected_mode:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000
    mov edi, 0x10000
    xor eax, eax
    mov ecx, 0xC00
    rep stosd
    mov dword [0x10000], 0x11007
    mov dword [0x11000], 0x12007

    mov ecx, 4
    mov edi, 0x12000
    mov eax, 0x000087
.map_kernel_heap_loop:
    mov [edi], eax
    mov dword [edi + 4], 0
    add eax, 0x200000
    add edi, 8
    loop .map_kernel_heap_loop

    mov ecx, 4
    mov edi, 0x12028
    mov eax, [0x7000]
    or  eax, 0x87
.map_lfb_loop:
    mov [edi], eax
    mov dword [edi + 4], 0
    add eax, 0x200000
    add edi, 8
    loop .map_lfb_loop

    mov eax, 0x10000
    mov cr3, eax
    mov eax, cr4
    or  eax, 0x20
    mov cr4, eax
    mov ecx, 0xC0000080
    rdmsr
    or  eax, 0x100
    wrmsr
    mov eax, cr0
    or  eax, 0x80000000
    mov cr0, eax
    jmp 0x18:long_mode

[bits 64]

long_mode:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov rsp, 0x90000
    mov rsi, 0x20000
    mov rdi, 0x100000
    mov rcx, 455168
    rep movsb
    mov rax, 0x100000
    jmp rax