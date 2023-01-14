CREATE TABLE `user` (
  `id` int NOT NULL AUTO_INCREMENT,
  `password` varchar(100) DEFAULT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `companies` (
  `id` int NOT NULL AUTO_INCREMENT,
  `last_updated` datetime DEFAULT NULL,
  `active` tinyint DEFAULT NULL,
  `charmtalk` tinyint DEFAULT NULL,
  `name` varchar(200) DEFAULT NULL,
  `description` varchar(1000) DEFAULT NULL,
  `unique_selling_point` varchar(1000) DEFAULT NULL,
  `summer_job_description` varchar(1000) DEFAULT NULL,
  `summer_job_link` varchar(1000) DEFAULT NULL,
  `summer_job_deadline` date DEFAULT NULL,
  `contacts` varchar(100) DEFAULT NULL,
  `contact_email` varchar(320) DEFAULT NULL,
  `employees_world` int DEFAULT NULL,
  `employees_sweden` int DEFAULT NULL,
  `website` varchar(200) DEFAULT NULL,
  `talk_to_us_about` varchar(1000) DEFAULT NULL,
  `logo` varchar(100) DEFAULT NULL,
  `map_image` int DEFAULT NULL,
  `booth_number` int DEFAULT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `tags` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(200) DEFAULT NULL,
  `parent_tag` int DEFAULT NULL,
  `up_votes` int DEFAULT NULL,
  `down_votes` int DEFAULT NULL,
  `crowd_sourced` tinyint DEFAULT NULL,
  `icon` varchar(100) DEFAULT NULL,
  `division` tinyint DEFAULT NULL,
  `business_area` tinyint DEFAULT NULL,
  `looking_for` tinyint DEFAULT NULL,
  `offering` tinyint DEFAULT NULL,
  `language` tinyint DEFAULT NULL,
  `fair_area` tinyint DEFAULT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `companies_tags` (
  `company_id` int NOT NULL,
  `tag_id` int NOT NULL,
  PRIMARY KEY (`company_id`,`tag_id`),
  KEY `tag_id` (`tag_id`),
  CONSTRAINT `companies_tags_ibfk_1` FOREIGN KEY (`company_id`) REFERENCES `companies` (`id`),
  CONSTRAINT `companies_tags_ibfk_2` FOREIGN KEY (`tag_id`) REFERENCES `tags` (`id`)
);


CREATE TABLE `company_cards` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(100) DEFAULT NULL,
  `text` varchar(100) DEFAULT NULL,
  `active` tinyint DEFAULT NULL,
  PRIMARY KEY (`id`)
);


CREATE TABLE `feedback` (
  `id` int NOT NULL AUTO_INCREMENT,
  `title` varchar(50) DEFAULT NULL,
  `text` varchar(1000) DEFAULT NULL,
  `meta` varchar(1000) DEFAULT NULL,
  `received` datetime DEFAULT NULL,
  PRIMARY KEY (`id`)
);


CREATE TABLE `layout` (
  `id` int NOT NULL AUTO_INCREMENT,
  `image` varchar(100) DEFAULT NULL,
  `active` tinyint DEFAULT NULL,
  `placement` int DEFAULT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `maps` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(100) DEFAULT NULL,
  `image` varchar(100) DEFAULT NULL,
  `ref` int DEFAULT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `prepages` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(100) DEFAULT NULL,
  `image` varchar(100) DEFAULT NULL,
  `active` tinyint DEFAULT NULL,
  `mobile` tinyint DEFAULT NULL,
  `side` varchar(5) DEFAULT NULL,
  `page` int DEFAULT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `shortcuts` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(100) DEFAULT NULL,
  `desc` varchar(100) DEFAULT NULL,
  `link` varchar(100) DEFAULT NULL,
  `icon` varchar(100) DEFAULT NULL,
  PRIMARY KEY (`id`)
);
