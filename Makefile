all: db.sqlite

clean:
	rm -f db.sqlite
	rm -rf assets/scripts/{build,doc,types}

db.sqlite:
	sqlite3 db.sqlite < schema.sql

.PHONY: scripts
scripts:
	cargo run --bin gen_types --features scripting
	cd assets/scripts && tl build
