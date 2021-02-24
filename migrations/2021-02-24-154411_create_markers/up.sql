CREATE TABLE markers (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(255) NOT NULL,
    latitude float NOT NULL,
    longitude float NOT NULL,
    zoom SMALLINT NOT NULL,
    page_id INTEGER NOT NULL,
    page_name VARCHAR(255) NOT NULL,
    page_revid INTEGER NOT NULL
);

CREATE INDEX ON markers (name);
CREATE INDEX ON markers (latitude);
CREATE INDEX ON markers (longitude);
CREATE INDEX ON markers (zoom);
CREATE INDEX ON markers (page_id);
CREATE INDEX ON markers (page_revid);
