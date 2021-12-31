use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: TokenSubject,
    pub claims: TokenClaimsData,
    #[serde(with = "ts_seconds")]
    pub exp: DateTime<Utc>,
}

impl Default for TokenClaims {
    fn default() -> Self {
        TokenClaims {
            sub: TokenSubject::Server(String::new()),
            claims: TokenClaimsData::Server(TokenClaimsDataServer {
                guild: String::new(),
            }),
            exp: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TokenSubject {
    #[serde(rename = "server")]
    Server(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TokenClaimsData {
    #[serde(rename = "server")]
    Server(TokenClaimsDataServer),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenClaimsDataServer {
    pub guild: String,
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::dangerous_insecure_decode;

    use super::*;

    #[test]
    fn test_token_claims_decode() {
        dangerous_insecure_decode::<TokenClaims>("eyJhbGciOiJIUzI1NiJ9.eyJqdGkiOiI2YTQ5ODUwOS0yZTZiLTRlMTQtOTJiZi1iMjVkMjQ1YTA4Y2YiLCJzdWIiOnsic2VydmVyIjoiYTIwNDE3OTEtZGQ0ZC00ODZkLTllODQtNTExYWZkNzYwOTEzIn0sImNsYWltcyI6eyJzZXJ2ZXIiOnsiZ3VpbGQiOiI3MDA4ODgyNDc5MjgzNTY5MDUiLCJwZXJtcyI6WyJndWlsZHMubWVtYmVycy52aWV3IiwiZ3VpbGRzLm1lbWJlcnMuY3JlYXRlIiwiZ3VpbGRzLnJvbGVzLnZpZXciLCJndWlsZHMuZXZlbnRzLnZpZXciLCJndWlsZHMuYXR0ZW5kYW5jZS52aWV3IiwiZ3VpbGRzLmF0dGVuZGFuY2UuY3JlYXRlIiwiZ3VpbGRzLmF0dGVuZGFuY2UuZGVsZXRlIiwiZ3VpbGRzLmF0dGVuZGFuY2UuZWRpdCIsImd1aWxkcy5jb3Vyc2VzLnZpZXciLCJndWlsZHMuaW5zdHJ1Y3RvcnMudmlldyIsImd1aWxkcy50cmlhbHMudmlldyIsImd1aWxkcy50cmlhbHMuY3JlYXRlIiwiZ3VpbGRzLnNlcnZlcnMudmlldyIsImd1aWxkcy5zZXJ2ZXJzLmNyZWF0ZSIsImd1aWxkcy5zZXJ2ZXJzLmVkaXQiXX19LCJpYXQiOjE2Mzk2MjI0NDEsImV4cCI6MTY0MjIxNDQ0MSwiaXNzIjoiZGlzY29yZDozMDc1MjQwMDk4NTQxMDc2NDgifQ.sTyWQAdb8nvdoiPgKEhVkY3gJcvEjTmlFRTO54Hg3bQ").unwrap();
    }
}
