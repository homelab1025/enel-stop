--liquibase formatted sql

--changeset author:florin id:005

ALTER TABLE incidents DROP COLUMN datetime;

--rollback
-- ALTER TABLE incidents ADD COLUMN datetime VARCHAR(255) NOT NULL;