-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE reactions (
  id SERIAL PRIMARY KEY,
  product_id INTEGER REFERENCES products (id) ON DELETE CASCADE NOT NULL,
  user_id INTEGER REFERENCES users (id) ON DELETE CASCADE NOT NULL,
  kind VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL 
)
