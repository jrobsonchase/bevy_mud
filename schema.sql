CREATE TABLE entity (
	id integer not null primary key
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
