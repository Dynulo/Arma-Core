use std::{
    fs::File,
    io::{Read, Write},
    sync::RwLock,
};

use arma_rs::{Context, Group};
use jsonwebtoken::dangerous_insecure_decode;

use crate::token::{TokenClaims, TokenClaimsData};

pub fn group() -> Group {
    Group::new()
        .group(
            "token",
            Group::new()
                .command("register", token_register)
                .command("exists", token_exists),
        )
        .command("ready", ready)
}

const TOKEN_FILE: &str = "dynulo_token.txt";

lazy_static::lazy_static! {
    pub static ref HOST: String = format!("{}/api/v1", std::env::var("DYNULO_HOST")
        .unwrap_or_else(|_| "https://dev.dynulo.com".to_string()));
    pub static ref TOKEN: RwLock<String> = RwLock::new(String::new());
    pub static ref GUILD: RwLock<String> = RwLock::new(String::new());
    static ref READY: RwLock<bool> = RwLock::new(false);
    static ref CLAIM: RwLock<TokenClaims> = RwLock::new(TokenClaims::default());
}

fn token_register(ctx: Context, token: String) -> bool {
    if decode_token(token.as_str()) {
        let mut file = File::create(TOKEN_FILE).unwrap();
        file.write_all(token.as_bytes()).unwrap();
        info!("token saved to file");
        if !*READY.read().unwrap() {
            *READY.write().unwrap() = true;
            ctx.callback(
                "dynulo_core",
                "core:ready",
                Some(GUILD.read().unwrap().clone()),
            );
        }
        true
    } else {
        false
    }
}

fn token_exists() -> bool {
    !TOKEN.read().unwrap().is_empty()
}

fn ready(ctx: Context) {
    if token_exists() {
        *READY.write().unwrap() = true;
        ctx.callback(
            "dynulo_core",
            "core:ready",
            Some(GUILD.read().unwrap().clone()),
        );
    }
}

pub fn init() {
    if let Ok(mut file) = File::open(TOKEN_FILE) {
        let mut token = String::new();
        file.read_to_string(&mut token).unwrap();
        if decode_token(token.as_str()) {
            info!("token loaded from file");
        }
    } else {
        info!("no token file found, waiting for register");
    }
}

fn decode_token(token_str: &str) -> bool {
    let token = dangerous_insecure_decode::<TokenClaims>(token_str);
    if let Ok(token) = token {
        let claims = token.claims;
        *CLAIM.write().unwrap() = claims.clone();
        let TokenClaimsData::Server(server) = claims.claims;
        *GUILD.write().unwrap() = server.guild;
        *TOKEN.write().unwrap() = token_str.to_string();
        true
    } else {
        false
    }
}
