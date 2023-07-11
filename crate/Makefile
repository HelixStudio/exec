CC := clang
_CFLAGS := -std=c11 -g -Wall
CFLAGS := -std=c11 -O2
LDFLAGS := -lseccomp 
TARGET := crate
SRC := $(wildcard *.c)
OBJ := $(SRC:.c=.o)

all: build clean

build: $(OBJ)
	$(CC) $(LDFLAGS) -o $(TARGET) $^

%.o: %.c
	$(CC) -o $@ -c $^ $(_CFLAGS)

clean:
	rm -rf $(OBJ)

run:
	./$(TARGET)

install: all
	cp ./$(TARGET) /usr/bin/$(TARGET)