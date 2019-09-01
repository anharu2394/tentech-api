ALTER TABLE products ADD COLUMN simple varchar DEFAULT '';
ALTER TABLE products ALTER COLUMN simple SET NOT NULL;
