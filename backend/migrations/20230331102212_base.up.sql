CREATE TABLE users (
  id serial NOT NULL ,
  password varchar(100) DEFAULT NULL NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE companies (
  id serial NOT NULL NOT NULL,
  last_updated timestamp DEFAULT NULL NOT NULL,
  active boolean DEFAULT NULL NOT NULL,
  charmtalk boolean DEFAULT NULL NOT NULL,
  name varchar(200) DEFAULT NULL NOT NULL,
  description varchar(2000) DEFAULT NULL NOT NULL,
  unique_selling_point varchar(1000) DEFAULT NULL NOT NULL,
  summer_job_description varchar(1000) DEFAULT NULL NOT NULL,
  summer_job_link varchar(1000) DEFAULT NULL NOT NULL,
  summer_job_deadline date DEFAULT NULL NOT NULL,
  contacts varchar(100) DEFAULT NULL NOT NULL,
  contact_email varchar(320) DEFAULT NULL NOT NULL,
  employees_world int DEFAULT NULL NOT NULL,
  employees_sweden int DEFAULT NULL NOT NULL,
  website varchar(200) DEFAULT NULL NOT NULL,
  talk_to_us_about varchar(1000) DEFAULT NULL NOT NULL,
  logo varchar(200) DEFAULT NULL NOT NULL,
  map_image int DEFAULT NULL NOT NULL,
  booth_number int DEFAULT NULL NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE tags (
  id serial NOT NULL NOT NULL,
  name varchar(200) DEFAULT NULL NOT NULL,
  parent_tag int DEFAULT NULL NOT NULL,
  up_votes int DEFAULT NULL NOT NULL,
  down_votes int DEFAULT NULL NOT NULL,
  crowd_sourced boolean DEFAULT NULL NOT NULL,
  icon varchar(100) DEFAULT NULL NOT NULL,
  division boolean DEFAULT NULL NOT NULL,
  business_area boolean DEFAULT NULL NOT NULL,
  looking_for boolean DEFAULT NULL NOT NULL,
  offering boolean DEFAULT NULL NOT NULL,
  language boolean DEFAULT NULL NOT NULL,
  fair_area boolean DEFAULT NULL NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE companies_tags (
  company_id int NOT NULL,
  tag_id int NOT NULL
);
-- CREATE unique companies_tags_unique on companies_tags (company_id, tag_id); TODO FIX THIS SHIT
CREATE index company_id_index on companies_tags (company_id);
CREATE index tag_id_index on companies_tags (tag_id);


CREATE TABLE company_cards (
  id serial NOT NULL,
  name varchar(100) DEFAULT NULL NOT NULL,
  text varchar(100) DEFAULT NULL NOT NULL,
  active boolean DEFAULT NULL NOT NULL,
  PRIMARY KEY (id)
);


CREATE TABLE feedback (
  id serial NOT NULL,
  title varchar(50) DEFAULT NULL NOT NULL,
  text varchar(1000) DEFAULT NULL NOT NULL,
  meta varchar(1000) DEFAULT NULL NOT NULL,
  received timestamp DEFAULT NULL NOT NULL,
  important boolean DEFAULT FALSE NOT NULL,
  archived boolean DEFAULT FALSE NOT NULL,
  PRIMARY KEY (id)
);


CREATE TABLE layouts (
  id serial NOT NULL,
  image varchar(100) DEFAULT NULL NOT NULL,
  active boolean DEFAULT NULL NOT NULL,
  placement int DEFAULT NULL NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE maps (
  id serial NOT NULL,
  name varchar(100) DEFAULT NULL NOT NULL,
  image varchar(100) DEFAULT NULL NOT NULL,
  reference int DEFAULT NULL NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE prepages (
  id serial NOT NULL,
  name varchar(100) DEFAULT NULL NOT NULL,
  image varchar(100) DEFAULT NULL NOT NULL,
  active boolean DEFAULT NULL NOT NULL,
  mobile boolean DEFAULT NULL NOT NULL,
  side varchar(5) DEFAULT NULL NOT NULL,
  page int DEFAULT NULL NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE shortcuts (
  id serial NOT NULL,
  name varchar(100) NOT NULL,
  description varchar(100) NOT NULL,
  link varchar(100) NOT NULL,
  icon varchar(100) NOT NULL,
  PRIMARY KEY (id)
);
