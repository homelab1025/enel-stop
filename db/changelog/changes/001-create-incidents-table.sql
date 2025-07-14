--liquibase formatted sql

--changeset author:initial-schema id:001
--comment: Create incidents table based on the Incident struct

CREATE TABLE incidents
(
    id          VARCHAR(255) PRIMARY KEY,
    datetime    VARCHAR(255) NOT NULL,
    day         DATE,
    county      VARCHAR(255) NOT NULL,
    location    VARCHAR(255) NOT NULL,
    description TEXT         NOT NULL
);

CREATE INDEX incident_day ON incidents (day);
CREATE INDEX incident_county ON incidents (county);

--rollback DROP TABLE incidents;