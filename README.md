# Timeseries Aggregation Demo 

## Summary

This 0

## API

### Data Ingestion 


## Running locally

### Steps

A docker compose file has been provided with a timescaledb docker image (postgres + timescaledb extension) referenced. 
As the docker compose file(s) contain test secrets in raw text these should not be used for production deployment without modification. 

#### Running the service with cargo
To start the docker compose file in detached mode, run
``` shell
docker compose up -d
```

Then run the providing the database url/connection string as an environment variable, as follows: 
```shell
 TIMESERIES_AGGREGATION_DATABASE_URL=postgresql://postgres:testPassword@localhost:5432/timeseries cargo run 
```

#### Running the service with docker compose (build on demand)
To use docker compose to build and run the service and database
``` shell
docker compose -f docker-compose.yaml -f docker-compose-service.yaml up -d --build
```

### Environment Variables



## Assumptions

## Production Deployment Considerations

The following aspects of production deployment should be considered if deploying the service to. 

* Containerisation. A dockerfile has been created for the service with a non-root user. 
* Secrets management. The database has a username and password which should be kept secret and passed in 
through an environment variable. For the sake of local testing, this can be provided to the service container
on the command line or through docker compose as in the instructions. For production deployment, this should be 
done through 
* Authorization. It is likely that the queryable data and history will be confidential to some extent (i.e. due to commercial sensitivity as a bare minimum)
and should be protected using authn, e.g. through a JWT or API key. This has been left out for the time being due to
time constraints.

## Areas for Improvement

* Authorization. As mentioned in the production deployment considerations section above, if this was to go into production
it should include API endpoint protection using some suitable method of authorization. 
* Input data buffering. If real data is loaded from files rather than test data - which, as specified, 
could otherwise be treated as static data - this should be read in through a buffer rather than loading the whole file
at once to prevent memory issues where  the file to import becomes large. In real production systems, making assumptions 
about the size of such files ahead of time can lead to instability at runtime. Likewise, the insertion of the data into 
the SQL table (or a temporary table and moved if for consistency's sake all data needs to be made available at once) 
could be done in batches for similar reasons (e.g. buffer 1000 rows, insert 1000 rows until file is done). 
* Add page query parameter to query and history endpoints. 