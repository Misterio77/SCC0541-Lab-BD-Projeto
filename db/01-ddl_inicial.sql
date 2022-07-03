BEGIN;

CREATE TABLE airports (
    ident character(7) NOT NULL,
    type character(15),
    name text,
    latdeg double precision,
    longdeg double precision,
    elevft integer,
    continent character(2),
    isocountry character(2),
    isoregion character(7),
    city text,
    scheduled_service character(3),
    gpscode character(4),
    iatacode character(3),
    localcode character(7),
    homelink text,
    wikipedialink text,
    keywords text,
    CONSTRAINT airportpk PRIMARY KEY (ident)
);

COMMENT ON TABLE airports IS 'Airports in the World. HTML: https://ourairports.com/data/.  The primary key for interoperability purposes with other datasets is ident, but the actual internal OurAirports primary key is id.';
COMMENT ON COLUMN airports.ident IS 'Text identifier. It is the ICAO code if available. Otherwise, it will be a local airport code (if no conflict), or if nothing else is available, an internally-generated code starting with the ISO2 country code, followed by a dash and a four-digit number.';
COMMENT ON COLUMN airports.type IS 'Type of the airport. Allowed values: "closed_airport", "heliport", "large_airport", "medium_airport", "seaplane_base", or "small_airport".';
COMMENT ON COLUMN airports.name IS 'Official airport name, including "Airport", "Airstrip", etc.';
COMMENT ON COLUMN airports.latdeg IS 'The airport latitude in decimal degrees (positive for north).';
COMMENT ON COLUMN airports.longdeg IS 'The airport longitude in decimal degrees (positive for east).';
COMMENT ON COLUMN airports.elevft IS 'The airport elevation MSL in feet (not metres).';
COMMENT ON COLUMN airports.continent IS 'Continent where the airport is (primarily) located. Allowed values: "AF" (Africa), "AN" (Antarctica), "AS" (Asia), "EU" (Europe), "NA" (North America), "OC" (Oceania), or "SA" (South America).';
COMMENT ON COLUMN airports.isocountry IS 'Two-character ISO 3166:1-alpha2 code for the country. A handful of unofficial, non-ISO codes are also in use, such as "XK" for Kosovo. Refers to the Code column in countries.csv.';
COMMENT ON COLUMN airports.isoregion IS 'Alphanumeric code for the high-level administrative subdivision of a country where the airport is primarily located (e.g. province, governorate), prefixed by the ISO2 country code and a hyphen.';
COMMENT ON COLUMN airports.city IS 'The primary municipality that the airport serves (when available). This may not be the municipality where the airport is physically located.';
COMMENT ON COLUMN airports.scheduled_service IS '"yes" if the airport currently has scheduled airline service; "no" otherwise.';
COMMENT ON COLUMN airports.gpscode IS 'The code that an aviation GPS database (such as Jeppesen or Garmin) would normally use for the airport. This will always be the ICAO code if one exists. Unlike the Ident column, this is not guaranteed to be globally unique.';
COMMENT ON COLUMN airports.iatacode IS 'Three-letter IATA code (if it has one).';
COMMENT ON COLUMN airports.localcode IS 'Local country code, if different from GPSCode and IATACode fields (used mainly for US airports).';
COMMENT ON COLUMN airports.homelink IS 'URL of the airport''s official home page on the web, if exists.';
COMMENT ON COLUMN airports.wikipedialink IS 'URL of the airport''s page on Wikipedia, if exists.';
COMMENT ON COLUMN airports.keywords IS 'Extra keywords/phrases to assist with search, comma-separated. May include former names for the airport, alternate codes, names in other languages, nearby tourist destinations, etc. ';

CREATE TABLE circuits (
    circuitid integer NOT NULL,
    circuitref text NOT NULL,
    name text NOT NULL,
    location text,
    country text,
    lat double precision,
    lng double precision,
    alt double precision,
    url text,
    CONSTRAINT circuits_circuitref_key UNIQUE (circuitref),
    CONSTRAINT circuits_name_key UNIQUE (name),
    CONSTRAINT circuits_pkey PRIMARY KEY (circuitid)
);

CREATE TABLE constructors (
    constructorid integer NOT NULL,
    constructorref text,
    name text,
    nationality text,
    url text,
    CONSTRAINT constructors_constructorref_key UNIQUE (constructorref),
    CONSTRAINT constructors_name_key UNIQUE (name),
    CONSTRAINT constructors_pkey PRIMARY KEY (constructorid)
);

CREATE TABLE countries (
    code character(2),
    name text,
    continent character(2),
    keywords text
);

COMMENT ON TABLE countries IS 'Countries in the World. HTML: https://ourairports.com/data/.';
COMMENT ON COLUMN countries.code IS 'Two-character ISO 3166:1-alpha2 code for the country. A handful of unofficial, non-ISO codes are also in use, such as "XK" for Kosovo. The iso_country field in Countries.csv Points into this column.';
COMMENT ON COLUMN countries.name IS 'Common English-language name for the country. Other variations of the name may appear in the keywords field to assist with search.';
COMMENT ON COLUMN countries.continent IS 'Code for the continent where the country is (primarily) located. Allowed values: "AF" (Africa), "AN" (Antarctica), "AS" (Asia), "EU" (Europe), "NA" (North America), "OC" (Oceania), or "SA" (South America).';
COMMENT ON COLUMN countries.keywords IS 'Comma-separated list of search keywords/phrases related to the country.';

CREATE TABLE driver (
    driverid integer NOT NULL,
    driverref text,
    number integer,
    code text,
    forename text,
    surname text,
    dob date,
    nationality text,
    url text,
    CONSTRAINT driver_driverref_key UNIQUE (driverref),
    CONSTRAINT driver_pkey PRIMARY KEY (driverid),
    CONSTRAINT driver_url_key UNIQUE (url),
    CONSTRAINT drlogkey UNIQUE (forename, surname)
);

CREATE TABLE seasons (
    year integer NOT NULL,
    url text,
    CONSTRAINT seasons_pkey PRIMARY KEY (year)
);

CREATE TABLE races (
    raceid integer NOT NULL,
    year integer,
    round integer,
    circuitid integer,
    name text,
    date date,
    "time" text,
    url text,
    CONSTRAINT races_pkey PRIMARY KEY (raceid),
    CONSTRAINT races_url_key UNIQUE (url),
    CONSTRAINT fk_circuits FOREIGN KEY (circuitid) REFERENCES circuits(circuitid),
    CONSTRAINT fk_season FOREIGN KEY (year) REFERENCES seasons(year)
);

CREATE TABLE driverstandings (
    driverstandingsid integer NOT NULL,
    raceid integer,
    driverid integer,
    points double precision,
    "position" integer,
    positiontext text,
    wins integer,
    CONSTRAINT driverstandings_pkey PRIMARY KEY (driverstandingsid),
    CONSTRAINT dslogkey UNIQUE (raceid, driverid),
    CONSTRAINT dspositionkey UNIQUE (raceid, "position"),
    CONSTRAINT fk_driver FOREIGN KEY (driverid) REFERENCES driver(driverid),
    CONSTRAINT fk_race FOREIGN KEY (raceid) REFERENCES races(raceid)
);

CREATE TABLE geocities15k (
    geonameid integer,
    name text,
    asciiname text,
    alternatenames text,
    lat numeric(13,5),
    long numeric(13,5),
    featureclass character(1),
    featurecode text,
    country character(2),
    cc2 text,
    admin1code text,
    admin2code text,
    admin3code text,
    admin4code text,
    population bigint,
    elevation bigint,
    dem bigint,
    timezone text,
    modification date
);

COMMENT ON TABLE geocities15k IS 'Cities around the worlds with Population>15000 inhabitants or is a Capital. Obtained from:  http://download.geonames.org/export/dump/, file cities15000.zip ';
CREATE TABLE laptimes (
    raceid integer NOT NULL,
    driverid integer NOT NULL,
    lap integer NOT NULL,
    "position" integer,
    "time" text,
    milliseconds integer,
    CONSTRAINT laptimes_pkey PRIMARY KEY (raceid, driverid, lap),
    CONSTRAINT fk_driver FOREIGN KEY (driverid) REFERENCES driver(driverid),
    CONSTRAINT fk_race FOREIGN KEY (raceid) REFERENCES races(raceid)
);

CREATE TABLE pitstops (
    raceid integer NOT NULL,
    driverid integer NOT NULL,
    stop integer NOT NULL,
    lap integer,
    "time" text,
    duration text,
    milliseconds integer,
    CONSTRAINT pitstops_pkey PRIMARY KEY (raceid, driverid, stop),
    CONSTRAINT fk_driver FOREIGN KEY (driverid) REFERENCES driver(driverid),
    CONSTRAINT fk_race FOREIGN KEY (raceid) REFERENCES races(raceid)
);

CREATE TABLE qualifying (
    qualifyid integer NOT NULL,
    raceid integer,
    driverid integer,
    constructorid integer,
    number integer,
    "position" integer,
    q1 text,
    q2 text,
    q3 text,
    CONSTRAINT qualifying_pkey PRIMARY KEY (qualifyid),
    CONSTRAINT qulogkey UNIQUE (raceid, driverid, constructorid),
    CONSTRAINT fk_constructor FOREIGN KEY (constructorid) REFERENCES constructors(constructorid),
    CONSTRAINT fk_driver FOREIGN KEY (driverid) REFERENCES driver(driverid),
    CONSTRAINT fk_race FOREIGN KEY (raceid) REFERENCES races(raceid)
);

CREATE TABLE status (
    statusid integer NOT NULL,
    status text,
    CONSTRAINT status_pkey PRIMARY KEY (statusid)
);

CREATE TABLE results (
    resultid integer NOT NULL,
    raceid integer,
    driverid integer,
    constructorid integer,
    number integer,
    grid integer,
    "position" integer,
    positiontext text,
    positionorder integer,
    points double precision,
    laps integer,
    "time" text,
    milliseconds integer,
    fastestlap integer,
    rank integer,
    fastestlaptime text,
    fastestlapspeed text,
    statusid integer,
    CONSTRAINT results_pkey PRIMARY KEY (resultid),
    CONSTRAINT fk_constructor FOREIGN KEY (constructorid) REFERENCES constructors(constructorid),
    CONSTRAINT fk_driver FOREIGN KEY (driverid) REFERENCES driver(driverid),
    CONSTRAINT fk_race FOREIGN KEY (raceid) REFERENCES races(raceid),
    CONSTRAINT fk_status FOREIGN KEY (statusid) REFERENCES status(statusid)
);

CREATE VIEW Tables AS
    SELECT 'Circuits'             AS Table, Count(*) NroTuplas FROM Circuits UNION
    SELECT 'Constructors'         AS Table, Count(*) NroTuplas FROM Constructors UNION
    SELECT 'DriverStandings'      AS Table, Count(*) NroTuplas FROM DriverStandings UNION
    SELECT 'Driver'               AS Table, Count(*) NroTuplas FROM Driver UNION
    SELECT 'LapTimes'             AS Table, Count(*) NroTuplas FROM LapTimes UNION
    SELECT 'PitStops'             AS Table, Count(*) NroTuplas FROM PitStops UNION
    SELECT 'Qualifying'           AS Table, Count(*) NroTuplas FROM Qualifying UNION
    SELECT 'Races'                AS Table, Count(*) NroTuplas FROM Races UNION
    SELECT 'Results'              AS Table, Count(*) NroTuplas FROM Results UNION
    SELECT 'Seasons'              AS Table, Count(*) NroTuplas FROM Seasons UNION
    SELECT 'Status'               AS Table, Count(*) NroTuplas FROM Status UNION
    SELECT 'Airports'             AS Table, Count(*) NroTuplas FROM Airports UNION
    SELECT 'Countries'            AS Table, Count(*) NroTuplas FROM Countries UNION
    SELECT 'GeoCities15K'         AS Table, Count(*) NroTuplas FROM GeoCities15K;
COMMIT;
