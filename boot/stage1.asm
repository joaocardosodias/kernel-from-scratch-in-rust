[bits 16]
[org 0x7C00]

_start:
    cli
    xor ax,ax
    mov ds,ax
    mov es,ax
    mov ss,ax
    mov sp,0x7C00
    sti
    mov [drive_num],dl
    mov ah,0x02
    mov al,2
    mov ch,0
    mov cl,2
    mov dh,0
    mov dl,[drive_num]
    mov bx,0x7E00
    int 0x13
    jc disk_error
    jmp 0x0000:0x7E00

disk_error:
    mov si,msg_error
    call print_string
    hlt

msg_error:db "Disk Error!",0

print_string:
    mov ah,0x0E
.next_char:
    lodsb
    cmp al,0
    je .done
    int 0x10
    jmp .next_char
.done:
    ret

drive_num db 0
times 510-($-$$) db 0

dw 0xAA55