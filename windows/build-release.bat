@echo off
cd /d %~d0%~p0
cd ..
xargo build --release --target=gba
cd windows
bin\arm-none-eabi-as.exe -o ..\out\crt0.o ..\crt0.s
bin\arm-none-eabi-ld.exe -T ..\linker.ld -o ..\out\snake.elf ..\out\crt0.o ..\target\gba\release\libgba_snake.a
bin\arm-none-eabi-objcopy.exe -O binary ..\out\snake.elf ..\out\snake.gba
cd ..