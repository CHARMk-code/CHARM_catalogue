CREATE TABLE blobs (
  id serial NOT NULL,
  name varchar(50) NOT NULL DEFAULT '',
  blob JSON NOT NULL DEFAULT '{}',
  PRIMARY KEY (id) 
);
