# Setup

Setup 3 containers:
- nginx for the reverse proxy web server
- wordpress for the wordpress php application server
- mysql for the wordpress persistent storage


# Execution

## Startup

Startup the docker containers:
```docker-compose up```

this starts up all three containers and makes the web application reachable under port `:8081`.

If the port is already in use, you can change it at `docker-compose.yaml:35`.

On first startup you have to visit the website and complete the initial wordpress startup, including creating an admin user.

Then go to `/wp-admin` and navigate to `Tools -> Import`, download the WordPress import plugin. After that click on "Run Importer" and import the [sample file](https://raw.githubusercontent.com/WPTT/theme-test-data/master/themeunittestdata.wordpress.xml).


## HTTP Benchmark

To execute the http benchmark run:

```
./benchmark.sh
```

This benchmark retrieves the front page and measures the requests per second. 


## Full System profiling 

Unfortunately we didn't get full system profiling to work in time.


# Plots 
