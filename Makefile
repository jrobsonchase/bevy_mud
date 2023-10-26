clean:
	rm db.sqlite

db.sqlite:
	sqlite3 db.sqlite < schema.sql