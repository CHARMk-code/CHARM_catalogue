ALTER TABLE companies 
DROP CONSTRAINT fk_companies_map_image;

ALTER TABLE companies_tags
DROP CONSTRAINT fk_companies_tags_company_id;

ALTER TABLE companies_tags
DROP CONSTRAINT fk_companies_tags_tags_id;
