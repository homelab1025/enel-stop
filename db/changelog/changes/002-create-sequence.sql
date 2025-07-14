--liquibase formatted sql

--changeset author:florin id:002

CREATE SEQUENCE incidents_id
    INCREMENT BY 1
    MINVALUE 1
    MAXVALUE 9223372036854775807
    START 1
	CACHE 1
	NO CYCLE;

--rollback
-- DROP SEQUENCE incidents_id;