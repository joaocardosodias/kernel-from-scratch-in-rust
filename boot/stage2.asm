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

    mov ah, 0x02
    mov al, 32
    mov ch, 0
    mov cl, 3
    mov dh, 0
    mov dl, [drive_num]
    mov bx, 0x2000
    mov es, bx
    xor bx, bx
    int 0x13

    lgdt [gdt_ptr]
    cli
    mov eax, cr0
    or  eax, 1
    mov cr0, eax
    jmp 0x08:protected_mode

drive_num: db 0

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

    mov dword [0x10000], 0x11003
    mov dword [0x11000], 0x12003
    mov dword [0x12000], 0x000083

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
    mov rcx, 16384
    rep movsb

    mov rax, 0x100000
    jmp rax