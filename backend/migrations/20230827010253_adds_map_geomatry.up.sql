DROP TABLE maps; -- This is a destrative operation and as should it can be completely reverted

CREATE TABLE fair_maps(
  id serial NOT NULL,
  name varchar(100) NOT NULL,
  background varchar(512) NOT NULL,
  styling jsonb NOT NULL,
  PRIMARY KEY(id)
);


CREATE TABLE map_geomarty(
  id serial NOT NULL,
  map_ref int NOT NULL,
  postion point NOT NULL,
  category varchar(20) NOT NULL,
  refId int NOT NULL,
  styling jsonb NOT NULL,
  PRIMARY KEY(id),
  check (category in ('COMPANY', 'OTHER'))
);

CREATE INDEX map_geomarty_index ON map_geomarty(map_ref);
