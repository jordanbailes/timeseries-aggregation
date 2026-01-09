-- Your SQL goes here
CREATE TABLE timeseries (
                       time TIMESTAMPTZ NOT NULL PRIMARY KEY,
                       quantity_kwh DOUBLE PRECISION NOT NULL
);

CREATE TABLE query_history (id SERIAL PRIMARY KEY,
                      query_time TIMESTAMP NOT NULL DEFAULT current_timestamp,
                      from_date DATE,
                      to_date DATE,
                      granularity TEXT
);