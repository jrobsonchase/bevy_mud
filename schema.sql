CREATE TABLE user (
	  id integer not null primary key
	, name text not null unique
	, email text
	, password text not null
) strict;
CREATE TABLE entity (
	  id integer not null primary key
	, parent integer
	, constraint fk_entity_parent
	  foreign key (parent)
	  references entity(id)
	  on delete cascade
) strict;
CREATE TABLE component (
	  id integer not null primary key
	, name text not null unique
) strict;
CREATE TABLE entity_component (
	  entity integer not null
	, component integer not null
	, data text not null
	, primary key (entity, component)
	, constraint fk_ec_entity
	  foreign key (entity)
	  references entity(id)
	  on delete cascade
	, constraint fk_ec_component
	  foreign key (component)
	  references component(id)
	  on delete cascade
) strict;
