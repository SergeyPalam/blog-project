use argon2::Argon2;
use argon2::password_hash::{Error, PasswordHasher, SaltString, rand_core::OsRng};

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}
