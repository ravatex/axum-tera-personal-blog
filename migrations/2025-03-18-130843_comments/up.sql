-- Your SQL goes here
CREATE TABLE comments (
  post_id INTEGER NOT NULL REFERENCES posts ON DELETE CASCADE ON UPDATE CASCADE,
  id INTEGER NOT NULL,
  name VARCHAR NOT NULL,
  date DATE NOT NULL,
  contents VARCHAR NOT NULL,
  PRIMARY KEY(id,post_id)
)
