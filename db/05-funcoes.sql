-- Funções encapsulando métricas, relatórios, e ações
BEGIN;

DROP FUNCTION IF EXISTS admin_metrics;
DROP FUNCTION IF EXISTS report_1;
DROP FUNCTION IF EXISTS report_2;

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

-- TODO: índice?
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

COMMIT;
