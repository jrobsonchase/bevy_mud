all: db.sqlite base-db.sqlite build

clean:
	rm -f db.sqlite
	rm -rf assets/scripts/{build,doc,types}

base-db.sqlite: schema.sql
	rm -f base-db.sqlite
	sqlite3 base-db.sqlite < schema.sql

db.sqlite:
	sqlite3 db.sqlite < schema.sql

.PHONY: build
build:
	cargo build --release --features=bevy/dynamic_linking,tracy

.PHONY: stop
stop:
	pkill canton || true

.PHONY: start
start: build stop
	RUST_LOG=canton::framerate=debug,info cargo run --features=bevy/dynamic_linking,tracy --release &