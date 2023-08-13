# CHARMcatalogue

A fully fledged digital catalogue for the CHARM fair at Chalmers. The system is built to be easily usable and customizable by an administrator with no programming understanding. The current published version in production should be available on http://catalogue.charm.chalmers.se . The vision is easy for students to find the companies that are relevent to them, with is an issue in the paper version due to the amount of companies that attend the fair.


# Setup
## Production

### Requirements

A system configured with `Docker`

### Running 

Download and the `production.yml` file found in `.` and run it with `docker compose up` 


## Development

### Requirements

A system configured with `Docker`, `cargo` and `yarn`

### Running 
The database is started by simple running 
```
docker compose up 
```

To start the backend service go into `backend/` and run 
```
cargo run
```

For the frontend go in to the `vue/` directorty and run 
```
yarn dev 
```



## Configuration
The system can configured either via the `config.toml` or by setting enviroment varibles, if a value is not set in on of this ways then it will use default values. Values set via enviroment variable override those in the file, this to provide an easy way to configure producation enviroments through docker-compose files.

### Parameters
| Name | Default | Purpose|
|-|-|-|
|database_url || This specifics the how to connect to the database including url, port, and which database on the given server |
|upload_path | upload/ | This specifics where upload file should be temparily storage while processing |
|storage_path | storage/ | This specifics where upload file should be storage while serving them |
|password_salt|NOT A GOOD SALT| This should be a random generated string, which is used to improve password security |

The enviroment variables have the same name but uppercase.

## Usage
### Batch uploading

When the system is setup it can easily be populated by uploading a `.zip` file containing a `.xlsx` file along with all the images (named correctly) as an administrator. 

An example of such a file can be found [here](https://drive.google.com/drive/folders/1ARqpngACz8koJlrudFBCM7jHow94vemY?usp=sharing)

