-- Add position column to chapters table
ALTER TABLE chapters 
ADD COLUMN IF NOT EXISTS position INTEGER;

-- Copy existing order_no values to position
UPDATE chapters 
SET position = order_no 
WHERE position IS NULL;

-- Make position NOT NULL
ALTER TABLE chapters 
ALTER COLUMN position SET NOT NULL;-- Add migration script here
