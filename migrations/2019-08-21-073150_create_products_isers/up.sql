-- Your SQL goes here
CREATE TABLE products_tags (
  id SERIAL PRIMARY KEY,
  product_id INTEGER REFERENCES products NOT NULL,
  tag_id INTEGER REFERENCES tags NOT NULL
)
