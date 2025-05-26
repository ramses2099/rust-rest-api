-- Add migration script here
CREATE TABLE
    IF NOT EXISTS blog_posts (
        id SERIAL PRIMARY KEY,
        title VARCHAR(150) NOT NULL,
        author VARCHAR(150) NOT NULL,
        content VARCHAR(500) NOT NULL
    )
