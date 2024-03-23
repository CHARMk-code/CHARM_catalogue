use std::collections::HashMap;
use std::future::{ready, Ready};
use thiserror::Error;

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest, ResponseError};
use sqlx::postgres::PgPoolOptions;
use sqlx::{query, Pool, Postgres};

use crate::models::map;

#[derive(Clone)]
pub struct Tenant {
    pub name: String,
    pub db: Pool<Postgres>,
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to read tenants from app data")]
    NoTenants,

    #[error("Choosen tenant does not exist")]
    MissingTenant,

    #[error("Database could not be created")]
    FailedToCreateDatabase,

    #[error("Could not connect to database at all")]
    CouldNotConnect,

    #[error("No catalogue suffix for database connection string ")]
    MissingCatalogueSuffixOnDbUrl
}

impl ResponseError for DatabaseError {}

async fn initiate_pool(db_uri: &str, schema_name: &str) -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&(db_uri.clone().to_owned() + schema_name))
        .await
        .expect(format!("Failed to initialize Database pool {schema_name}").as_str());

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect(format!("Migrations failed, for {schema_name}").as_str());

    pool
}

async fn create_db(name: &str, pool: &Pool<Postgres>) -> Result<(), DatabaseError> {
    let db_exist = query!("SELECT FROM pg_database WHERE datname = $1", name)
        .execute(pool)
        .await
        .map_err(|_| DatabaseError::FailedToCreateDatabase)
        .map(|result| result.rows_affected() > 0)?;

    if !db_exist {
        query(format!("CREATE DATABASE {}", name).as_str())
            .execute(pool)
            .await
            .map_err(|_| DatabaseError::FailedToCreateDatabase)?;
    };

    Ok(())
}

pub async fn initialize_tenant_pools(
    db_uri: String,
) -> Result<HashMap<String, Tenant>, DatabaseError> {
    let db_base_uri = db_uri.strip_suffix("catalogue").ok_or(DatabaseError::MissingCatalogueSuffixOnDbUrl)?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_base_uri)
        .await
        .map_err(|_| DatabaseError::CouldNotConnect)?;

    println!("Db_uri {:?}", db_base_uri);

    create_db("charm", &pool).await?;
    create_db("date_it", &pool).await?;
    create_db("local", &pool).await?;

    let mut tenants: HashMap<String, Tenant> = HashMap::new();

    let default_pool = initiate_pool(db_base_uri, "local");
    let charm_pool = initiate_pool(db_base_uri, "charm");
    let date_it_pool = initiate_pool(db_base_uri, "date_it");

    tenants.insert(
        "catalogue.charm.chalmers.se".to_string(),
        Tenant {
            name: "charm".to_string(),
            db: charm_pool.await,
        },
    );
    tenants.insert(
        "catalogue.date-it.se".to_string(),
        Tenant {
            name: "date_it".to_string(),
            db: date_it_pool.await,
        },
    );
    tenants.insert(
        "localhost".to_string(),
        Tenant {
            name: "local".to_string(),
            db: default_pool.await,
        },
    );

    Ok(tenants)
}

impl FromRequest for Tenant {
    type Error = DatabaseError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> <Self as FromRequest>::Future {
        ready((|| {
            let tenants: &HashMap<String, Tenant> = req
                .app_data::<HashMap<String, Tenant>>()
                .ok_or(DatabaseError::NoTenants)?;

            let origin_option = req.uri().host();

            let tenant = if let Some(origin) = origin_option {
                if tenants.contains_key(origin) {
                    tenants.get(origin)
                } else {
                    tenants.get("localhost")
                }
            } else {
                tenants.get("localhost")
            };

            match tenant {
                None => Err(DatabaseError::MissingTenant),
                Some(t) => Ok(t.to_owned()),
            }
        })())
    }
}
