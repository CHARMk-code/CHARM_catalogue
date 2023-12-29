-- clean up unused columns
ALTER TABLE tags DROP COLUMN parent_tag,
    DROP COLUMN up_votes,
    DROP COLUMN down_votes,
    DROP COLUMN crowd_sourced;


-- Create new categories
CREATE TABLE tag_category(
    id SERIAL NOT NULL,
    name VARCHAR(100) NOT NULL,
    PRIMARY KEY(id)
);

-- Add reference
ALTER TABLE tags ADD category SERIAL NOT NULL;
ALTER TABLE tags ADD CONSTRAINT fk_tag_category FOREIGN KEY (category) REFERENCES tag_category(id);

/*
INSERT INTO tag_category VALUES (0, 'division'),
(1, 'business_area'),
(2, 'looking_for'),
(3, 'offering'),
(4, 'language'),
(5, 'fair_area');

UPDATE tags SET category = 0 WHERE division;
UPDATE tags SET category = 1 WHERE business_area;
UPDATE tags SET category = 2 WHERE looking_for;
UPDATE tags SET category = 3 WHERE offering;
UPDATE tags SET category = 4 WHERE language;
UPDATE tags SET category = 5 WHERE fair_area;
*/

-- Clean up old tag flags
ALTER TABLE tags
    DROP COLUMN division,
    DROP COLUMN business_area,
    DROP COLUMN looking_for,
    DROP COLUMN offering,
    DROP COLUMN language,
    DROP COLUMN fair_area;
