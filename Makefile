all: test batch watch repl

SRC = game.rs batch.rs ai.rs repl.rs watch.rs
test: $(SRC)
	rustc batch.rs --test -o test

batch: $(SRC)
	rustc batch.rs --opt-level=3 -o batch

watch: $(SRC)
	rustc watch.rs --opt-level=3 -o watch

repl: $(SRC)
	rustc repl.rs -o repl

.PHONY: clean

clean:
	rm -f test
	rm -f batch
	rm -f watch
	rm -f repl
