BEGIN;

DROP TRIGGER IF EXISTS autoregister ON driver;
DROP FUNCTION IF EXISTS register_driver_trigger;

DROP TRIGGER IF EXISTS autoregister ON constructors;
DROP FUNCTION IF EXISTS register_constructor_trigger;

DROP FUNCTION IF EXISTS register_driver;
DROP FUNCTION IF EXISTS register_constructor;

DROP TABLE IF EXISTS users;
DROP TYPE IF EXISTS user_type;

-- Tipo usuário e tabela

CREATE TYPE user_type AS ENUM ('Administrador', 'Escuderia', 'Piloto');

CREATE TABLE users (
    userid integer NOT NULL GENERATED ALWAYS AS IDENTITY,
    login text NOT NULL,
    password text NOT NULL,
    tipo user_type NOT NULL,
    idoriginal integer,
    CONSTRAINT userpk PRIMARY KEY (userid),
    CONSTRAINT users_logink UNIQUE (login)
);

-- Funções para registrar constructors e drivers

CREATE FUNCTION register_constructor(c constructors) RETURNS void AS $$
DECLARE
    user_exists bool;
BEGIN
    -- Verificar se o usuário já existe
    SELECT EXISTS(
        SELECT idoriginal FROM users
            WHERE idoriginal = c.constructorid
            AND tipo = 'Escuderia'
    ) INTO user_exists;

    IF user_exists THEN
        RAISE EXCEPTION 'The constructor "%" is already a registered user', c.constructorref;
    END IF;

    INSERT INTO users (userid, login, password, tipo, idoriginal) VALUES (
        DEFAULT,
        (c.constructorref || '_c'),
        md5(c.constructorref),
        'Escuderia',
        c.constructorid
    );
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION register_driver(d driver) RETURNS void AS $$
DECLARE
    user_exists bool;
BEGIN
    -- Verificar se o usuário já existe
    SELECT EXISTS(
        SELECT idoriginal FROM users
            WHERE idoriginal = d.driverid
            AND tipo = 'Piloto'
    ) INTO user_exists;

    IF user_exists THEN
        RAISE EXCEPTION 'The driver "%" is already a registered user', d.driverref;
    END IF;

    INSERT INTO users (userid, login, password, tipo, idoriginal) VALUES (
        DEFAULT,
        (d.driverref || '_d'),
        md5(d.driverref),
        'Piloto',
        d.driverid
    );
END;
$$ LANGUAGE plpgsql;

-- Triggers utilizando as funções acima

-- Constructors
CREATE FUNCTION register_constructor_trigger() RETURNS trigger AS $$
BEGIN
    PERFORM register_constructor(NEW);
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER autoregister AFTER INSERT ON constructors
    FOR EACH ROW EXECUTE FUNCTION register_constructor_trigger();

-- Drivers
CREATE FUNCTION register_driver_trigger() RETURNS trigger AS $$
BEGIN
    PERFORM register_driver(NEW);
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER autoregister AFTER INSERT ON driver
    FOR EACH ROW EXECUTE FUNCTION register_driver_trigger();

COMMIT;
