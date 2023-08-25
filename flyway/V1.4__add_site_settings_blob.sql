CREATE TABLE `blobs` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(50) DEFAULT NULL,
  `blob` JSON NOT NULL DEFAULT ("{}"),
  PRIMARY KEY (`id`)
)
  
