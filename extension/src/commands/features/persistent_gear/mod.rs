use arma_rs::Group;

mod bank;
mod player;
mod shop;

pub fn group() -> Group {
    Group::new()
        .group("bank", bank::group())
        .group("shop", shop::group())
        .group("player", player::group())
}
