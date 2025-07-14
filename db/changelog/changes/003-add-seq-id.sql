--liquibase formatted sql

--changeset author:florin id:003

ALTER TABLE incidents DROP CONSTRAINT IF EXISTS incidents_pkey;
ALTER TABLE incidents RENAME COLUMN id TO external_id;
ALTER TABLE incidents ADD COLUMN id BIGINT DEFAULT nextval('incidents_id');
ALTER TABLE incidents ALTER COLUMN id SET NOT NULL;
ALTER TABLE incidents ADD PRIMARY KEY (id);

--rollback
-- ALTER TABLE incidents DROP CONSTRAINT IF EXISTS incidents_pkey;
-- ALTER TABLE incidents DROP COLUMN IF EXISTS id;
-- ALTER TABLE incidents RENAME COLUMN external_id TO id;
-- ALTER TABLE incidents ADD PRIMARY KEY (id);