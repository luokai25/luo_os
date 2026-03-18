CC      = gcc
CFLAGS  = -m32 -ffreestanding -O2 -Wall -Wextra -fno-stack-protector -fno-pic
LDFLAGS = -m elf_i386 -T linker.ld 

GRUB_CFG = isodir/boot/grub/grub.cfg
ISO      = luo_os.iso

all: $(ISO)

boot/boot.o: boot/boot.asm
	nasm -f elf32 boot/boot.asm -o boot/boot.o

kernel/kernel.o: kernel/kernel.c
	$(CC) $(CFLAGS) -c kernel/kernel.c -o kernel/kernel.o

kernel.bin: boot/boot.o kernel/kernel.o
	ld $(LDFLAGS) -o kernel.bin boot/boot.o kernel/kernel.o

$(GRUB_CFG):
	@mkdir -p isodir/boot/grub
	@echo 'set timeout=0'                    > $(GRUB_CFG)
	@echo 'set default=0'                   >> $(GRUB_CFG)
	@echo 'menuentry "luo_os" {'            >> $(GRUB_CFG)
	@echo '    multiboot /boot/kernel.bin'  >> $(GRUB_CFG)
	@echo '    boot'                        >> $(GRUB_CFG)
	@echo '}'                               >> $(GRUB_CFG)

$(ISO): kernel.bin $(GRUB_CFG)
	cp kernel.bin isodir/boot/kernel.bin
	grub-mkrescue -o $(ISO) isodir

run: $(ISO)
	qemu-system-i386 -cdrom $(ISO) -display curses -m 32M

clean:
	rm -f boot/boot.o kernel/kernel.o kernel.bin $(ISO)
