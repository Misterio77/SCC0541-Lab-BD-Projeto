-- Existem muitos campos nuláveis na base, que não tem nenhuma tupla com ele
-- NULL Então vou acrescentar mais NOT NULLs onde sinto que faz sentido

-- Também adicionamos generated no id de driver e constructor, pra possibilitar inserção

BEGIN;

ALTER TABLE constructors
    ALTER COLUMN constructorid ADD GENERATED ALWAYS AS IDENTITY,
    ALTER COLUMN constructorref SET NOT NULL,
    ALTER COLUMN name SET NOT NULL,
    ALTER COLUMN nationality SET NOT NULL;
SELECT setval(
    pg_get_serial_sequence('constructors', 'constructorid'),
    (SELECT max(constructorid) FROM constructors)
);

ALTER TABLE driver
    ALTER COLUMN driverid ADD GENERATED ALWAYS AS IDENTITY,
    ALTER COLUMN driverref SET NOT NULL,
    ALTER COLUMN forename SET NOT NULL,
    ALTER COLUMN surname SET NOT NULL,
    ALTER COLUMN dob SET NOT NULL,
    ALTER COLUMN nationality SET NOT NULL;
SELECT setval(
    pg_get_serial_sequence('driver', 'driverid'),
    (SELECT max(driverid) FROM driver)
);

COMMIT;
