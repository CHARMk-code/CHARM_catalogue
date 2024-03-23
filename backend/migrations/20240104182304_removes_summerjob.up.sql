ALTER TABLE companies 
DROP COLUMN summer_job_deadline,
DROP COLUMN summer_job_description,
DROP COLUMN summer_job_link;

DELETE FROM company_cards WHERE name = 'summerjob'
