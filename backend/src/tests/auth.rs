use jwt_simple::prelude::NoCustomClaims;

use crate::services::auth::{generate_token, hash_password, validate_token};

#[test]
pub fn hashing_of_password_should_be_the_same_every_time() -> Result<(), argon2::Error> {
    let password = "password_for_testing_hashing".to_string();

    let hashed_password1 = hash_password(&password)?;
    let hashed_password2 = hash_password(&password)?;

    assert_eq!(
        hashed_password1, hashed_password2,
        "both hashed passwords should be identical"
    );

    Ok(())
}

#[test]
pub fn generated_token_should_be_validateable_by_the_same_keypair() -> Result<(), jwt_simple::Error>
{
    let key_pair = jwt_simple::prelude::Ed25519KeyPair::generate();

    let token = generate_token(key_pair.clone());

    assert!(&token.is_ok());

    let claims = validate_token::<NoCustomClaims>(&token.unwrap(), key_pair);

    assert!(claims.is_ok());

    Ok(())
}

//TODO: Write tests for getting, updating and creating users. However wait for remake of
//authentication system before that
