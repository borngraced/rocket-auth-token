//Auth token and credentials helper for your rocket web api. This is a basic token generator crate for Rust Rocket Web. Let credentials = AuthCredentials{email: "email", password: "password"}, use hasher(credentials) function to generate and send AuthToken to user on login. User then need to supply the generated token as auth header when making request to server. Using rocket request guard, grab the auth token and decode with decode_hasher function.
///
///
///
use base64::{decode, encode};
use serde::Serialize;
/// This crate made use of external crate, base64 for encoding and decoding auth credentials
use std::error::Error;

/// AuthCredentials to be encoded into AuthToken and vice-versa
#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct AuthCredentials {
    pub email: String,
    pub password: String,
}

/// AuthToken is to be passed into Authorization header after encoding user credentials
#[derive(Debug, Serialize, Clone)]
pub struct AuthToken {
    pub token: String,
}

/// Encode AuthCredentials into base64 encoded string and return AuthToken
pub fn encode_credentials(auth: &AuthCredentials) -> AuthToken {
    let info = format!("{}:{}", auth.email, auth.password);
    let hashed = encode(info);
    AuthToken { token: hashed }
}

/// Decode passed AuthToken and return AuthCredentials
pub fn decode_token(hashed: &AuthToken) -> Result<AuthCredentials, Box<dyn Error>> {
    let decode = decode(&&hashed.token);

    //check if decode result is valid
    match decode {
        //if valid proceed with okay
        Ok(r) => {
            //Get [u8] from the valid result of AuthToken decoding
            //Extract String from r using String::from_utf8()
            let extract = String::from_utf8(r);

            // check extraction outcome
            match extract {
                Ok(info) => {
                    // if extraction is successful continue with Ok

                    //split info from token extraction and collect into a vec, usuaully, the result is always in this form "email.com:password"
                    let info_vec: Vec<_> = info.split(":").collect();

                    //Pass credentials to AuthCredentials
                    Ok(AuthCredentials {
                        email: info_vec[0].to_string(),
                        password: info_vec[1].to_string(),
                    })
                }

                // if extraction is unsuccessful return error
                Err(_) => return Err("Can't decode, try again".to_string().into()),
            }
        }
        // if decoding is unsuccessful return error
        Err(_) => return Err("Invalid token!".to_string().into()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode_token, encode_credentials, AuthCredentials};

    #[test]
    fn it_works() {
        let credentials = AuthCredentials {
            email: "my@mail.com".to_string(),
            password: "SamopE160!".to_string(),
        };
        let hash = encode_credentials(&credentials);
        let decode = decode_token(&hash).unwrap();

        assert_eq!(credentials, decode); //true
    }
}
