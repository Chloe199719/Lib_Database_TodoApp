-- This file should undo anything in `up.sql`
Alter table  email_verify_tokens DROP COLUMN expires_at;