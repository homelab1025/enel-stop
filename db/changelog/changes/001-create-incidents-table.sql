--liquibase formatted sql

--changeset author:initial-schema id:001
--comment: Create incidents table based on the Incident struct

--"{\"id\":\"132587 - Retele Electrice\",\"date\":\"2025-06-17\",\"judet\":\"CONSTANTA\",\"localitate\":\"CONSTANTA\",\"title\":\"17.06.2025 09:00 - 12:00  Judet: CONSTANTA Localitate: CONSTANTA\",\"description\":\"Strada: ZONA DEDEMAN,  CU BLOC LOCUINTE PERPETUM RESIDENCE III - STRADA DOBRICH NR. 10 - INTRERUPERE TOTALA.\\\\t Numar:\"}"

CREATE TABLE incidents
(
    id          VARCHAR(255) PRIMARY KEY,
    datetime    VARCHAR(255) NOT NULL,
    day         DATE,
    county      VARCHAR(255) NOT NULL,
    location    VARCHAR(255) NOT NULL,
    description TEXT         NOT NULL,
);

CREATE INDEX incident_day ON incidents (day);
CREATE INDEX incident_county ON incidents (county);

--rollback DROP TABLE incidents;