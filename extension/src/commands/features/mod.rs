use arma_rs::{Context, Group};

// mod courses;
pub mod persistent_gear;

use crate::{
    commands::core::{GUILD, HOST, TOKEN},
    worker::fn_task,
    QUEUE,
};

pub fn group() -> Group {
    Group::new()
        .command("fetch", fetch)
        .group("persistent-gear", persistent_gear::group())
    // .group("courses", courses::group())
}

fn fetch(ctx: Context) -> String {
    if let Ok(q) = QUEUE.lock() {
        let task = fn_task(move |id| {
            let client = reqwest::blocking::Client::new();
            let path = format!("{}/guild/{}/features", *HOST, GUILD.read().unwrap());
            info!("[{}] fetching features from {}", id, path);
            if let Ok(response) = client
                .get(path)
                .header("Authorization", TOKEN.read().unwrap().clone())
                .header("Content-Type", "application/json")
                .send()
            {
                if let Ok(json) = response.json::<Vec<i32>>() {
                    info!("[{}] fetched {} features", id, json.len());
                    ctx.callback("dynulo_core", "features:fetch", Some(json));
                } else {
                    warn!("[{}] failed to fetch features", id);
                }
            }
            Ok(())
        });
        let id = task.id().to_string();
        q.push(task);
        id
    } else {
        error!("Failed to lock queue");
        "".to_string()
    }
}
