use arma_rs::Group;

mod bank;
mod player;
mod shop;

lazy_static::lazy_static! {
    pub static ref HOST: String = format!("{}/persistent-gear/api/v3", std::env::var("DYNULO_HOST")
        .unwrap_or_else(|_| "https://dev.dynulo.com".to_string()));
}

pub fn group() -> Group {
    Group::new()
        .group("bank", bank::group())
        .group("shop", shop::group())
        .group("player", player::group())
}
