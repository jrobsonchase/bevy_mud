all: db.sqlite scripts

clean:
	rm -f db.sqlite
	rm -rf assets/scripts/{build,doc,types}

db.sqlite:
	sqlite3 db.sqlite < schema.sql

.PHONY: scripts
scripts:
	cargo run --bin gen_types
	cd assets/scripts && tl build
