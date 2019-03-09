SOURCES := $(wildcard *.js)
OUTS := $(patsubst %.js,%,$(SOURCES))

all: $(OUTS)

$(OUTS): %: %.js
	-./rjsc $< -o $@.o

clean: 
	rm -rf *.c
	rm -rf *.json
	rm -rf *.o
	rm -rf *.h
	rm -rf *.gch

.PHONY: all clean
