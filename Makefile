all: db.sqlite

clean:
	rm -f db.sqlite
	rm -rf assets/scripts/{build,doc,types}

db.sqlite:
	sqlite3 db.sqlite < schema.sql
