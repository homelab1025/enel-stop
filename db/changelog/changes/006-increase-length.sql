--liquibase formatted sql

--changeset author:florin id:006

ALTER TABLE incidents ALTER COLUMN location TYPE TEXT;

--rollback
-- ALTER TABLE incidents ALTER COLUMN location TYPE VARCHAR(255);