ALTER TABLE services ADD COLUMN image
VARCHAR(100);
ALTER TABLE services ADD COLUMN description
VARCHAR(300);


ALTER TABLE organizations_places DROP COLUMN district_id;
ALTER TABLE organizations_places DROP COLUMN lat;
ALTER TABLE organizations_places DROP COLUMN lon;
ALTER TABLE organizations_places ADD COLUMN address2
VARCHAR(300) NOT NULL DEFAULT '';
ALTER TABLE organizations_places ALTER COLUMN city_id
SET NOT NULL;

ALTER TABLE services DROP COLUMN city_id;