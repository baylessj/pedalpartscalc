-- Your SQL goes here
CREATE TYPE part_kind AS ENUM (
    'Resistor',
    'Capacitor',
    'Transistor',
    'Diode',
    'Potentiometer',
    'Switch'
);

CREATE TABLE available_parts (
  id bigint PRIMARY KEY,
  owner_id bigint NOT NULL,
  part_name TEXT NOT NULL,
  quantity INTEGER NOT NULL,
  kind part_kind NOT NULL
);