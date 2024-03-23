ALTER TABLE
ADD COLUMN summer_job_description varchar(1000) DEFAULT NULL NOT NULL,
ADD COLUMN summer_job_link varchar(1000) DEFAULT NULL NOT NULL,
ADD COLUMN summer_job_deadline timestamptz DEFAULT NULL NOT NULL;

INSERT INTO company_cards (text, name, active)
VALUES
  ('Summer job', 'summerjob', true),
