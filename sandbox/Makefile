SOURCES := $(wildcard *.js)
OUTS := $(patsubst %.js,%,$(SOURCES))

all: $(OUTS)

$(OUTS): %: %.js
	-./rjsc $< -o $@.o
	-./rjsc -a $<

clean: 
	rm -rf *.c
	rm -rf *.s
	rm -rf *.json
	rm -rf *.o
	rm -rf *.h
	rm -rf *.gch

.PHONY: all clean
