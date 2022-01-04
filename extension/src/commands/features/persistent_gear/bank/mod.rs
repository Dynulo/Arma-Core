use arma_rs::Group;

mod balance;
mod purchases;

pub fn group() -> Group {
    Group::new()
        .group("balance", balance::group())
        .group("purchases", purchases::group())
}
