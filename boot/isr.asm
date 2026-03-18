bits 32
extern keyboard_handler
extern timer_handler

global keyboard_isr
global timer_isr

timer_isr:
    pusha
    call timer_handler
    popa
    iret

keyboard_isr:
    pusha
    call keyboard_handler
    popa
    iret

section .note.GNU-stack noalloc noexec nowrite progbits
