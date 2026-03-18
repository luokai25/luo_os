bits 32
global context_switch

context_switch:
    mov eax, [esp+4]
    mov [eax+0],  esp
    mov [eax+4],  ebp
    mov [eax+8],  ebx
    mov [eax+12], esi
    mov [eax+16], edi
    pushfd
    pop dword [eax+20]

    mov eax, [esp+8]
    mov esp, [eax+0]
    mov ebp, [eax+4]
    mov ebx, [eax+8]
    mov esi, [eax+12]
    mov edi, [eax+16]
    push dword [eax+20]
    popfd
    ret

section .note.GNU-stack noalloc noexec nowrite progbits
