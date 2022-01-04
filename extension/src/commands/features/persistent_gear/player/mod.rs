use arma_rs::Group;

mod loadout;
mod locker;

pub fn group() -> Group {
    Group::new()
        .group("loadout", loadout::group())
        .group("locker", locker::group())
}
