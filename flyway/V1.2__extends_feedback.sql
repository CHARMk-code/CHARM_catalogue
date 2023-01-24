ALTER TABLE `feedback`
    ADD `important` tinyint DEFAULT 0,
    ADD `archived` tinyint DEFAULT 0;