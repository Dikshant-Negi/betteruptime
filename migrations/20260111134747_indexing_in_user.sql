-- Add migration script here
CREATE INDEX indx_email ON users(email);

CREATE INDEX indx_website_user ON websites(user_id);
