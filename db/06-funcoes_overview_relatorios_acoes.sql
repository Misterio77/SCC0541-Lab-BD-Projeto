-- Funções encapsulando métricas, relatórios, e ações
BEGIN;

/*
DROP FUNCTION IF EXISTS admin_metrics;
DROP FUNCTION IF EXISTS report_1;
DROP FUNCTION IF EXISTS report_2;
DROP INDEX IF EXISTS airports_type_isocountry;

DROP FUNCTION IF EXISTS constructor_metrics;
DROP FUNCTION IF EXISTS report_3;
DROP FUNCTION IF EXISTS report_4;
DROP INDEX IF EXISTS results_constructorid;

DROP FUNCTION IF EXISTS drivers_by_forename;
DROP FUNCTION IF EXISTS driver_metrics;
DROP FUNCTION IF EXISTS report_5;
DROP FUNCTION IF EXISTS report_6;
DROP INDEX IF EXISTS results_driverid;
DROP INDEX IF EXISTS results_position_driverid;
*/

-- === ADMIN ===
CREATE FUNCTION admin_metrics()
    RETURNS TABLE (
        drivers bigint,
        constructors bigint,
        races bigint,
        seasons bigint
    )
AS $$
BEGIN
    RETURN QUERY SELECT
        (SELECT count(*) FROM driver) AS drivers,
        (SELECT count(*) FROM constructors) AS constructors,
        (SELECT count(*) FROM races) AS races,
        (SELECT count(*) FROM seasons) AS seasons;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION report_1()
    RETURNS TABLE (
        status text,
        count bigint
    )
AS $$
BEGIN
    RETURN QUERY SELECT s.status, count(*)
        FROM results r INNER JOIN status s ON s.statusid = r.statusid
        GROUP BY s.status, s.statusid ORDER BY s.statusid;
END;
$$ LANGUAGE plpgsql;

CREATE INDEX airports_type_isocountry ON airports (type, isocountry);
CREATE FUNCTION report_2(text)
    RETURNS TABLE (
        city_name text,
        airport_name text,
        airport_iata char(3),
        airport_city text
    )
AS $$
BEGIN
    RETURN QUERY SELECT
        c.name, a.name, a.iatacode, a.city
    FROM
        (SELECT name, iatacode, city, latdeg lat, longdeg long
            FROM airports
            WHERE (type = 'medium_airport' OR type = 'large_airport') AND isocountry = 'BR'
        ) a,
        (SELECT name, lat, long FROM geocities15k WHERE name = $1) c
    WHERE earth_distance(ll_to_earth(a.lat, a.long), ll_to_earth(c.lat, c.long))::int <= 100000;
END;
$$ LANGUAGE plpgsql;
-- === FIM ADMIN ===


-- === ESCUDERIA ===
CREATE FUNCTION constructor_metrics(int)
    RETURNS TABLE (
        wins bigint,
        drivers bigint,
        first_year int,
        last_year int
    )
AS $$
BEGIN
    RETURN QUERY SELECT
        (SELECT count(*) FROM results WHERE constructorid = $1 AND position = 1) AS wins,
        (SELECT count(DISTINCT driverid) FROM results WHERE constructorid = $1) AS drivers,
        (SELECT year FROM races
            INNER JOIN results ON results.raceid = races.raceid
            WHERE results.constructorid = $1
            ORDER BY year LIMIT 1) AS first_year,
        (SELECT year FROM races
            INNER JOIN results ON results.raceid = races.raceid
            WHERE results.constructorid = $1
            ORDER BY year DESC LIMIT 1) AS last_year;
END;
$$ LANGUAGE plpgsql;

CREATE INDEX results_constructorid ON results (constructorid); -- Esse ajuda também na metrics!
CREATE FUNCTION report_3(int)
    RETURNS TABLE (
        name text,
        wins bigint
    )
AS $$
BEGIN
    RETURN QUERY SELECT (forename || ' ' || surname) AS name,
        COUNT(*) FILTER (WHERE position = 1) AS wins
        FROM driver INNER JOIN results ON results.driverid = driver.driverid
        WHERE constructorid = $1
        GROUP BY name ORDER BY wins DESC;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION report_4(int)
    RETURNS TABLE (
        status text,
        count bigint
    )
AS $$
BEGIN
    RETURN QUERY SELECT s.status, count(*)
        FROM results r INNER JOIN status s ON s.statusid = r.statusid
        WHERE constructorid = $1
        GROUP BY s.status, s.statusid ORDER BY s.statusid;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION drivers_by_forename(int, text)
    RETURNS SETOF driver
AS $$
BEGIN
    RETURN QUERY SELECT DISTINCT driver.*
        FROM driver INNER JOIN results ON results.driverid = driver.driverid
        WHERE constructorid = $1 AND forename = $2
        ORDER BY surname;
END;
$$ LANGUAGE plpgsql;
-- === FIM ESCUDERIA ===

-- === MOTORISTA ===
CREATE FUNCTION driver_metrics(int)
    RETURNS TABLE (
        wins bigint,
        first_year int,
        last_year int
    )
AS $$
BEGIN
    RETURN QUERY SELECT
        (SELECT count(*) FROM results WHERE driverid = $1 AND position = 1) AS wins,
        (SELECT year FROM races
            INNER JOIN results ON results.raceid = races.raceid
            WHERE results.driverid = $1
            ORDER BY year LIMIT 1) AS first_year,
        (SELECT year FROM races
            INNER JOIN results ON results.raceid = races.raceid
            WHERE results.driverid = $1
            ORDER BY year DESC LIMIT 1) AS last_year;
END;
$$ LANGUAGE plpgsql;

CREATE INDEX results_driverid ON results (driverid); -- Esse funciona tanto no report quanto nas métricas
-- CREATE INDEX results_position_driverid ON results (position, driverid); -- Esse funciona só no report
CREATE FUNCTION report_5(int)
    RETURNS TABLE (
        year int,
        race text,
        wins bigint
    )
AS $$
BEGIN
    RETURN QUERY SELECT races.year, races.name, COUNT(*) AS wins
        FROM results INNER JOIN races
        ON results.raceid = races.raceid
        WHERE results.position = 1 AND results.driverid = $1
        GROUP BY ROLLUP (races.year, races.name)
        ORDER BY name NULLS FIRST, year NULLS FIRST;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION report_6(int)
    RETURNS TABLE (
        status text,
        count bigint
    )
AS $$
BEGIN
    RETURN QUERY SELECT s.status, count(*)
        FROM results r INNER JOIN status s ON s.statusid = r.statusid
        WHERE driverid = $1
        GROUP BY s.status, s.statusid ORDER BY s.statusid;
END;
$$ LANGUAGE plpgsql;
-- === FIM MOTORISTA ===

COMMIT;
