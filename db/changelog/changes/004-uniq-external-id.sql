--liquibase formatted sql

--changeset author:florin id:004

CREATE UNIQUE INDEX unique_external_id ON incidents external_id;

--rollback
-- DROP INDEX IF EXISTS unique_external_id