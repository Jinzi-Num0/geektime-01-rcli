use anyhow::Result;
use jsonwebtoken::{
    decode, encode,
    errors::{Error, ErrorKind},
    get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    device: String,
    exp: u64,
}

pub fn process_gwt_sign(sub: String, aud: String, exp: String) -> Result<String> {
    let exp = get_current_timestamp() + parser_time(&exp)?;
    let claims = Claims {
        sub,
        device: aud,
        exp,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;
    Ok(token)
}

pub fn process_gwt_verify(token: String) -> Result<String> {
    let token_message = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_message.claims.into())
}

impl From<Claims> for String {
    fn from(val: Claims) -> Self {
        format!("sub: {}, device: {}, exp: {}", val.sub, val.device, val.exp)
    }
}

//time parser: 13d/12h/11m/10s
fn parser_time(exp: &str) -> Result<u64> {
    let exp = exp.split('/');
    let mut time = 0;
    for t in exp {
        let t = t.trim();
        let (num, unit) = t.split_at(t.len() - 1);
        let num: u64 = num.parse()?;
        time += match unit {
            "d" => num * 24 * 60 * 60,
            "h" => num * 60 * 60,
            "m" => num * 60,
            "s" => num,
            _ => return Err(Error::from(ErrorKind::ExpiredSignature).into()),
        };
    }
    Ok(time)
}
