CC := clang
CFLAGS := -std=c99
# LDFLAGS := -lseccomp
LDFLAGS :=
TARGET := crate
SRC := $(wildcard *.c)
OBJ := $(SRC:.c=.o)

all: build clean

debug: CFLAGS += -g -Wall -DDEBUG
debug: all

build: $(OBJ)
	$(CC) $(LDFLAGS) -o $(TARGET) $^

%.o: %.c
	$(CC) -o $@ -c $^ $(CFLAGS)

clean:
	rm -rf $(OBJ)

run:
	./$(TARGET)

install: CFLAGS += -O2
install: all
	cp ./$(TARGET) /usr/bin/$(TARGET)
