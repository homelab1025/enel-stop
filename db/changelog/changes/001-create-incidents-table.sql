--liquibase formatted sql

--changeset author:initial-schema id:001
--comment: Create incidents table based on the Incident struct

CREATE TABLE incidents (
    id VARCHAR(255) PRIMARY KEY,
    county VARCHAR(255) NOT NULL,
    location VARCHAR(255) NOT NULL,
    datetime VARCHAR(255) NOT NULL,
    description TEXT NOT NULL
);

--rollback DROP TABLE incidents;