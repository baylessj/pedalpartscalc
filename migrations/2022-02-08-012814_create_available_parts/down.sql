-- This file should undo anything in `up.sql`
drop table available_parts;
delete from pg_type where typname = 'part_kind';