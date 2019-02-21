C := gcc
CFLAGS = -g -Wall -std=c99
.PHONY: test

test: test.o databox.o print.o
	$(CC) $(LDFLAGS)  $^ -o $@

lib: databox.c print.c
	$(CC) $(LDFLAGS)  $^ -c -fpic -o $@

run:
	./test

%.o: %.c %.h
	$(CC) $(CFLAGS) -c $<

%.o: %.c
	$(CC) $(CFLAGS) -c $<

clean:
	rm *.o test




