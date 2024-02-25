CREATE TABLE user (
	id integer not null primary key,
	name text not null unique,
	email text,
	password text not null
) strict;
CREATE TABLE character (
	id integer not null primary key,
	user_id integer not null references user(id),
	entity integer not null references entity(id)
) strict;
CREATE TABLE entity (
	id integer not null primary key,
	parent integer references entity(id) on delete cascade
) strict;
CREATE TABLE component (
	id integer not null primary key,
	name text not null unique
) strict;
CREATE TABLE entity_component (
	entity integer not null references entity(id) on delete cascade,
	component integer not null references component(id),
	data text not null,
	primary key (entity, component)
) strict;