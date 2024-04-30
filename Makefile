FEATURES = bevy/dynamic_linking,tracy
RUST_LOG = bevy_sqlite=debug,bevy_mud=debug,example=debug,info
CARGO_ARGS = --release

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
	DATABASE_URL=sqlite://base-db.sqlite cargo sqlx prepare -- --all-targets --all-features
	cd bevy_sqlite && DATABASE_URL=sqlite://base-db.sqlite cargo sqlx prepare -- --all-targets --all-features

.PHONY: build
build:
	cargo build $(CARGO_ARGS) --features=$(FEATURES)

.PHONY: stop
stop:
	pkill example || true

.PHONY: start
start: build stop
	RUST_LOG=$(RUST_LOG) cargo run --features=$(FEATURES) $(CARGO_ARGS)  &
