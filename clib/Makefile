CC_x64 := x86_64-w64-mingw32-gcc
CC_x86 := i686-w64-mingw32-gcc
OPTIONS := -masm=intel -Wall

program:
	@echo "[+] Starting build payload..."
	$(CC_x64) rc4.c uuid.c aes.c main.c -o enc.x64.exe $(OPTIONS)
	$(CC_x86) rc4.c uuid.c aes.c main.c -o enc.x86.exe $(OPTIONS)
