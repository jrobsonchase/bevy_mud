FEATURES = bevy/dynamic_linking,tracy
RUST_LOG = canton=debug,info

all: db.sqlite base-db.sqlite build

clean:
	rm -f db.sqlite
	rm -rf assets/scripts/{build,doc,types}

base-db.sqlite: schema.sql
	rm -f base-db.sqlite
	sqlite3 base-db.sqlite < schema.sql

db.sqlite:
	sqlite3 db.sqlite < schema.sql

.PHONY: prepare-sqlx
prepare-sqlx:
	DATABASE_URL=sqlite://base-db.sqlite cargo sqlx prepare --workspace -- --all-targets

.PHONY: build
build:
	cargo build --release --features=$(FEATURES)

.PHONY: stop
stop:
	pkill canton || true

.PHONY: start
start: build stop
	RUST_LOG=$(RUST_LOG) cargo run --features=$(FEATURES) --release &
