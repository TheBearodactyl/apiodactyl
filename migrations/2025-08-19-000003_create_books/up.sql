CREATE TABLE books (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    genres TEXT[] NOT NULL CHECK (array_position(genres, null) IS NULL),
    tags TEXT[] NOT NULL CHECK (array_position(tags, null) IS NULL),
    rating INTEGER NOT NULL CHECK (rating >= 0),
    status TEXT NOT NULL,
    description TEXT NOT NULL,
    my_thoughts TEXT NOT NULL,
    links JSONB,
    cover_image TEXT NOT NULL,
    explicit BOOLEAN NOT NULL,
    color TEXT
);
