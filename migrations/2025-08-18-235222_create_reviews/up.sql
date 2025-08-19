CREATE TABLE reviews (
  id SERIAL PRIMARY KEY,
  chapter INTEGER NOT NULL,
  description TEXT NOT NULL,
  rating INTEGER NOT NULL CHECK (rating >= 0),
  thoughts TEXT NOT NULL
);
