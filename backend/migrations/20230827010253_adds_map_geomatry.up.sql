DROP TABLE maps; -- This is a destrative operation and as should it can be completely reverted

CREATE TABLE fair_maps(
  id serial NOT NULL,
  name varchar(100) NOT NULL,
  background varchar(512) NOT NULL,
  map_data jsonb NOT NULL,
  PRIMARY KEY(id)
);
