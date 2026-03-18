bits 32
extern keyboard_handler

global keyboard_isr
keyboard_isr:
    pusha
    call keyboard_handler
    popa
    iret
