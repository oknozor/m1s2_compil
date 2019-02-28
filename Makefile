EXEMPLE=exemples

expression:
	cargo run $(EXEMPLE)/01-expressions.json
declaration:
	cargo run $(EXEMPLE)/02-declarations.json
while:
	cargo run $(EXEMPLE)/03-while.json
for:
	cargo run $(EXEMPLE)/04-if-while.json
while-break:
	cargo run $(EXEMPLE)/05-for.json
function:
	cargo run $(EXEMPLE)/16-funcs.json
