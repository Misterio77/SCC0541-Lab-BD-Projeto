-- Existem muitos campos nuláveis na base, que não tem nenhuma tupla com ele
-- NULL Então vou acrescentar mais NOT NULLs onde sinto que faz sentido

BEGIN;

ALTER TABLE constructors
    ALTER COLUMN constructorref SET NOT NULL,
    ALTER COLUMN name SET NOT NULL,
    ALTER COLUMN nationality SET NOT NULL;

ALTER TABLE driver
    ALTER COLUMN driverref SET NOT NULL,
    ALTER COLUMN forename SET NOT NULL,
    ALTER COLUMN surname SET NOT NULL,
    ALTER COLUMN dob SET NOT NULL,
    ALTER COLUMN nationality SET NOT NULL;

COMMIT;
