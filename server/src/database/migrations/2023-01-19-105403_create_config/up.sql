-- Your SQL goes here

CREATE TABLE "config" (
  id SERIAL PRIMARY KEY,
  firstname VARCHAR NOT NULL,
  lastname VARCHAR NOT NULL
);

INSERT INTO "config" (firstname, lastname) VALUES ('Bartholomé', 'Gili');