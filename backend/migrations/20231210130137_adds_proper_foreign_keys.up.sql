ALTER TABLE companies 
ADD CONSTRAINT fk_companies_map_image FOREIGN KEY (map_image) REFERENCES fair_maps(id);

ALTER TABLE companies_tags
ADD CONSTRAINT fk_companies_tags_company_id FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE;

ALTER TABLE companies_tags
ADD CONSTRAINT fk_companies_tags_tags_id FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE;
