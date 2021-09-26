use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{thread, time};

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
    test: String,
}

fn main() {
    let key = b"secret";
    let my_iat = Utc::now().timestamp();
    let my_exp = Utc::now()
        .checked_add_signed(Duration::seconds(5))
        .expect("invalid timestamp")
        .timestamp();
    let my_claims = Claims {
        sub: "h@d.com".to_owned(),
        iat: my_iat as usize,
        exp: my_exp as usize,
        test: "hello world".to_owned(),
    };

    let token = match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key),
    ) {
        Ok(t) => t,
        Err(_) => panic!(),
    };

    println!("token: {:?}", token);

    // println!("Waiting...");
    // thread::sleep(time::Duration::from_secs(6));
    let validator = Validation {
        sub: Some("h@d.com".to_owned()),
        ..Validation::default()
    };
    let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validator) {
        Ok(c) => c,
        Err(err) => {
            println!("err: {:?}", err.kind());
            panic!()
        }
    };

    println!("token data: {:?}", token_data);
}
