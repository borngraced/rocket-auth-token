Auth token and credentials helper for your rocket web api.

This is a basic token generator crate for Rust Rocket Web.

let credentials = AuthCredentials{email: "email", password: "password"},
use hasher(credentials) function to generate and send AuthToken to user on login.

user then need to supply the generated token as auth header when making request to server. Using rocket request guard, grab the auth token and decode with decode_hasher function.
