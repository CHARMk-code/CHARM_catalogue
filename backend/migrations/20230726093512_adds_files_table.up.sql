CREATE TABLE files (
  id UUID default gen_random_uuid() PRIMARY KEY,
  name varchar(128) NOT NULL,
  namespace varchar(128) default 'NONE' NOT NULL, 
  image boolean DEFAULT false NOT NULL
);
