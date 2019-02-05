SOURCES := $(wildcard *.js)
JSONS := $(patsubst %.js,%.json,$(SOURCES))

all: $(JSONS)

# Appelle le parseur de Babel sur le code source
# Ce parseur est relativement lent, autant l’appeler une fois pour
# toute
$(JSONS): %.json: %.js
	babylon $< > $@

# Pseudo-règle pour visualiser la partie la plus intéressante du
# résultat du parseur (filtre de nombreux champs)
$(patsubst %.js,%.ast,$(SOURCES)): %.ast: %.json
	jq '.program.body | del(..|.start?) | del(..|.end?) | del(..|.loc?) | del (..|.extra?) | del(..|.leadingComments?) | del(..|.trailingComments?)' < $<

move: 
	mv *.json ../examples

clean:
	rm -rf *.json

.PHONY: all clean
