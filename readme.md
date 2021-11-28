Auth token and credentials helper for your rocket web api.

This is a basic token generator crate for Rust Rocket Web.

let credentials = AuthCredentials{email: "email", password: "password"},
use hasher(credentials) function to generate AuthToken for user on login,
user then need to supply Authorization token as header when making request from server using rocket request guard, grab the auth token and decode with decode_hasher and return ok() if credentials is corrrect!
