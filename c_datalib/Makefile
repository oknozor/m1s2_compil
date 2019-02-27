C := gcc
CFLAGS = -g -Wall -std=c99

.PHONY: test clean

archive: databox.o print.o
	ar rsv lib_compil.a databox.o print.o

test: test.o databox.o print.o
	$(CC) $(LDFLAGS)  $^ -o $@
run:
	./test

%.o: %.c %.h
	$(CC) $(CFLAGS) -c $<

%.o: %.c
	$(CC) $(CFLAGS) -c $<

clean:
	rm -f *.o test
	rm -f lib_compil.a




