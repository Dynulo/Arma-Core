use arma_rs::Group;

mod items;

pub fn group() -> Group {
    Group::new().group("items", items::group())
}
