-- Esse script cria usuários para os constructors e drivers que já existem, e
-- também o usuário admin
BEGIN;

-- Criar conta p/ admin
INSERT INTO users (login, password, tipo) VALUES ('admin', md5('admin'), 'Administrador');
-- Criar contas p/ construtores
SELECT register_constructor(constructors.*) FROM constructors;
-- Criar contas p/ pilotos
SELECT register_driver(driver.*) FROM driver;

COMMIT;
