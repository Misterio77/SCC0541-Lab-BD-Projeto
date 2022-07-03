-- Esse script cria usuário para os constructors e drivers que já existem
BEGIN;

SELECT register_constructor(constructors.*) FROM constructors;
SELECT register_driver(driver.*) FROM driver;

COMMIT;
