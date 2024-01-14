-- Add up migration script here
ALTER TABLE companies
    ADD COLUMN image_office VARCHAR(200) NOT NULL,
    ADD COLUMN image_product VARCHAR(200) NOT NULL,
    ADD COLUMN founded INT NOT NULL,
    ADD COLUMN office_location VARCHAR(100) NOT NULL,
    ADD COLUMN male_board_share INT NOT NULL,
    ADD COLUMN female_board_share INT NOT NULL,
    ADD COLUMN nonbinary_board_share INT NOT NULL,
    ADD COLUMN qr_link VARCHAR(200) NOT NULL;
