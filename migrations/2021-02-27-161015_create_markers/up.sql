CREATE TABLE markers (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    latitude float NOT NULL,
    longitude float NOT NULL,
    zoom TINYINT NOT NULL,
    page_id INT NOT NULL,
    page_name VARCHAR(255) NOT NULL,
    page_revid INT NOT NULL
);

CREATE INDEX idx_name ON markers (name);
CREATE INDEX idx_latitude ON markers (latitude);
CREATE INDEX idx_longitude ON markers (longitude);
CREATE INDEX idx_zoom ON markers (zoom);
CREATE INDEX idx_page_id ON markers (page_id);
CREATE INDEX idx_page_revid ON markers (page_revid);
