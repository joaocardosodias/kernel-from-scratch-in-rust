[bits 16]
[org 0x7E00]

_start:
    cli
    xor  ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7E00
    sti

    xor ax, ax
    mov ds, ax
    mov es, ax

    lgdt [gdt_ptr]
    cli
    mov eax, cr0
    or  eax, 1
    mov cr0, eax
    jmp 0x08:protected_mode

msg:     db "Boot OK", 0
msg_len  equ $ - msg

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

    mov rdi, 0xB8000
    mov rsi, msg
    mov rcx, msg_len - 1
.write:
    mov al, [rsi]
    mov [rdi], al
    mov byte [rdi+1], 0x07
    add rdi, 2
    add rsi, 1
    loop .write

    cli
    jmp $