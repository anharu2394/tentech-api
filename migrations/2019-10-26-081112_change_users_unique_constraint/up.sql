ALTER TABLE users DROP CONSTRAINT users_password_key;
ALTER TABLE users ADD CONSTRAINT users_email_key UNIQUE (email);