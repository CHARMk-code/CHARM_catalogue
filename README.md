# CHARMcatalogue

A digital catalogue for the CHARM fair at Chalmers. The system is built foe ease of use and customizability by an administrator without programming background. The current published version in production should be available on [https://catalogue.charm.chalmers.se](https://catalogue.charm.chalmers.se).

The main goal is to make the relevant companies more accessible to students by offering all the functionality from the physical book along with functionality not possible to have in a book such as searching, filtering, and links.


# Setup
## Production

### Requirements

A system configured with `Docker`

### Running 

Either run the `production.yml` file found in `.` with `docker compose -f production.yml up`. This will build the containers from source allowing you to deploy any version easily.

Or you can make use of the package `charm_catalogue-backend` and `charm_catalogue-vue` (associated with this repo) along with a postgres database. For configuration either look at the `production.yml` file or the configuration instructions below


## Development

### Requirements

A system configured with `Docker`, `cargo` and `yarn`

For the backend to properly compile you need to add the `linux-musl` target to your toolchain
```rustup target add x86_64-unknown-linux-musl```

### Running 
The database is started by running 
```
docker compose up 
```

To start the backend service go into `backend/` and run 
```
cargo run
```

For the frontend go into the `vue/` directorty and run 
```
yarn dev 
```



## Configuration
The system can configured either via the `config.toml` or by setting enviroment variables. If the config file is incomplete a default config will be loadeed instead. Values set via enviroment variable override those in the file (or set by default), this to provide an easy way to configure a production enviroments through for example docker compose files.

### Parameters
| Name | Default | Purpose|
|-|-|-|
|database_url |  | This specifics the how to connect to the database including url, port, and which database on the given server. |
|upload_path | upload/ | This specifies where uploaded files should be temporarily saved while processing. |
|storage_path | storage/ | This specifies where uploade file should be saved after processing, in order to be served. |
|password_salt| NOT A GOOD SALT | This should be a random generated string, which is used to improve password security |

To set environment variables the same name but uppercase is used.

## Usage
### Batch uploading

When the system is setup it can easily be populated by uploading a `.zip` file containing a `.xlsx` file along with all the images (named correctly) as an administrator. 

An example of such a file can be found [here](https://drive.google.com/drive/folders/1ARqpngACz8koJlrudFBCM7jHow94vemY?usp=sharing)

