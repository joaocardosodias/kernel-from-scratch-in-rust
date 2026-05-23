

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

lgdt [gdt_ptr]
    cli
    mov eax,cr0
    or eax,1
    mov cr0,eax
    jmp 0x08:protected_mode

[bits 32]

protected_mode:
    mov ax,0x10
    mov ds,ax
    mov es,ax
    mov fs,ax
    mov gs,ax
    mov ss,ax

    mov esp,0x90000
    mov edi,0xB8000
    mov esi,msg
    mov ecx,msg_len
.write:
    mov al, [esi]
    mov [edi],al
    mov byte [edi+1],0x07
    add edi,2
    add esi,1
    loop .write
    jmp $

msg:db "BOOT OK-Protected Mode",0
gdt_start:
    dq 0

gdt_code:
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

gdt_end:

gdt_ptr:
    dw gdt_end -gdt_start -1 
    dd gdt_start
msg_len equ $ - msg