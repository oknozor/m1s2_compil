C := gcc
CFLAGS = -g -Wall -std=c99

.PHONY: test clean

archive: databox.o print.o
	ar rsv lib_compil.a databox.o print.o

test: test.o databox.o print.o
	$(CC) $(LDFLAGS)  $^ -o $@

dict_example: dict_example.c dict.o keyval.o
	$(CC) $(LDFLAGS) $^ -o $@


dict_example.o: keyval.o dict.o

run:
	./test

%.o: %.c %.h
	$(CC) $(CFLAGS) -c $<

%.o: %.c
	$(CC) $(CFLAGS) -c $<

clean:
	rm -f *.o test
	rm -f lib_compil.a
	rm -f dict_example




