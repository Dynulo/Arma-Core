use arma_rs::Group;

mod members;
mod roles;

pub fn group() -> Group {
    Group::new()
        .group("members", members::group())
        .group("roles", roles::group())
}
