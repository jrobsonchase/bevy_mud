all: db.sqlite base-db.sqlite

clean:
	rm -f db.sqlite
	rm -rf assets/scripts/{build,doc,types}

base-db.sqlite: schema.sql
	rm -f base-db.sqlite
	sqlite3 base-db.sqlite < schema.sql
db.sqlite:
	sqlite3 db.sqlite < schema.sql
