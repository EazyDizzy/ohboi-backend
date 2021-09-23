CREATE TYPE characteristic_group_slug AS ENUM ('processor','memory', 'connection', 'display', 'camera', 'sensors', 'power', 'appearance', 'general');

ALTER TABLE characteristic ADD COLUMN sort_key smallint NOT NULL DEFAULT 0;
ALTER TABLE characteristic ADD COLUMN group_slug characteristic_group_slug NOT NULL DEFAULT 'general';