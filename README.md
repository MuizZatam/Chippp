# Chippp

A CHIP-8 Emulator

# Organization

src/main.c is the main file that gets compiled
main.c includes various standard headers and
utility headers from src/utils

For testing/development, if an include file and
its contents are not detected, use `bear -- make`.
If the problem persists, check if SDL2 is correctly
installed:

```sh
# Debian specific

$ sudo apt install libsdl2-2.0-0
$ sudo apt install libsdl2-dev
```

# Notes to Self

- Dockerize in further development 
