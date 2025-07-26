SRC = src/main.c src/utils/*.c
CFLAGS = -std=c2x -Wall -Wextra -Werror -I./src/ `sdl2-config --cflags`
LIBS = `sdl2-config --libs`
OUT = build/chippp


all:
	gcc $(SRC) $(CFLAGS) $(LIBS) -o $(OUT)
