-- Your SQL goes here

CREATE TABLE special_abilities (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE teams (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE players (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  team_id SERIAL REFERENCES teams
);

CREATE TABLE player_abilities (
  id SERIAL PRIMARY KEY,
  player_id SERIAL REFERENCES players(id),
  special_ability_id SERIAL REFERENCES special_abilities(id)
);
