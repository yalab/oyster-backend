CREATE TABLE sites (
  id SERIAL PRIMARY KEY,
  url VARCHAR NOT NULL,
  crawled_at timestamp
);
