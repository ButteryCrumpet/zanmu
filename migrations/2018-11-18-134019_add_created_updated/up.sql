alter table todos add column created timestamp, add column updated timestamp;
alter table todos alter column created set default now();
alter table todos alter column updated set default now();