CC = gcc
TARGET = ilc
LIBS = -lzmq -lczmq
OPT = -O2 -flto
all:
	$(CC) $(OPT) ilc.c $(LIBS) -o $(TARGET)
clean:
	rm ilc