CC = gcc
CFLAGS = -Wall -Wextra -std=c99 -I source/headers
SRC = source/main.c source/includes/*.c
OUT = build/chippp
LIB = -lraylib -lm -ldl -lpthread -lGL -lX11

all:
	$(CC) $(SRC) $(CFLAGS) $(LIB) -o $(OUT) 
