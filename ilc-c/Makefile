CC = gcc
TARGET = ilc
FLAGS = -Wall
LIBS = -lzmq -lczmq
OPT = -O2 -flto
all:
	$(CC) $(FLAGS) $(OPT) ilc.c $(LIBS) -o $(TARGET)
clean:
	rm ilc