ALTER TABLE tags ADD division BOOLEAN DEFAULT NULL NOT NULL,
    ADD business_area BOOLEAN DEFAULT NULL NOT NULL,
    ADD looking_for BOOLEAN DEFAULT NULL NOT NULL,
    ADD offering BOOLEAN DEFAULT NULL NOT NULL,
    ADD language BOOLEAN DEFAULT NULL NOT NULL,
    ADD fair_area BOOLEAN DEFAULT NULL NOT NULL, 
    ADD parent_tag INT DEFAULT NULL NOT NULL,
    ADD up_votes int DEFAULT NULL NOT NULL,
    ADD down_votes INT DEFAULT NULL NOT NULL,
    ADD crowd_sourced BOOLEAN DEFAULT NULL NOT NULL;


UPDATE tags SET division = true WHERE category = 0
UPDATE tags SET business_area = true WHERE category = 1
UPDATE tags SET looking_for = true WHERE category = 2
UPDATE tags SET offering = true WHERE category = 3
UPDATE tags SET language = true WHERE category = 4
UPDATE tags SET fair_area = true WHERE category = 5


-- Add reference
ALTER TABLE tags DROP CONSTRAINT fk_tag_category;
ALTER TABLE tags DROP COLUMN category;

-- Create new categories
DROP TABLE tag_category;