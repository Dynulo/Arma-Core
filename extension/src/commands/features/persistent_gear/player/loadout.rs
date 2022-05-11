use arma_rs::{Context, Group};
use reqwest::StatusCode;

use crate::{
    commands::core::{GUILD, HOST, TOKEN},
    worker::fn_task,
    QUEUE,
};

pub fn group() -> Group {
    Group::new().command("fetch", fetch).command("store", store)
}

fn fetch(ctx: Context, discord: String, steam: String) -> String {
    if let Ok(q) = QUEUE.lock() {
        let task = fn_task(move |id| {
            let steam = steam.clone();
            let client = reqwest::blocking::Client::new();
            let path = format!(
                "{}/guild/{}/features/persistent_gear/player/{}/loadout",
                *HOST,
                *GUILD.read().unwrap(),
                discord
            );
            info!("[{}] fetching loadout from {}", id, path);
            if let Ok(response) = client
                .get(path)
                .header("Authorization", TOKEN.read().unwrap().clone())
                .header("Content-Type", "application/json")
                .send()
            {
                match response.status() {
                    StatusCode::OK => match response.text() {
                        Ok(text) => {
                            info!("[{}] fetched loadout", id);
                            ctx.callback(
                                "dynulo_core",
                                "features:persistent-gear:player:loadout:fetch",
                                Some(vec!["loaded".to_string(), steam, text]),
                            );
                        }
                        Err(e) => {
                            error!("[{}] failed to fetch loadout: {}", id, e);
                            ctx.callback(
                                "dynulo_core",
                                "features:persistent-gear:player:loadout:fetch",
                                Some(vec!["error".to_string(), steam]),
                            );
                        }
                    },
                    StatusCode::NO_CONTENT => {}
                    s => {
                        error!("[{}] failed to fetch loadout: {}", id, s);
                        ctx.callback(
                            "dynulo_core",
                            "features:persistent-gear:player:loadout:fetch",
                            Some(vec!["error".to_string(), steam]),
                        );
                    }
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

fn store(ctx: Context, discord: String, steam: String, loadout: String) -> String {
    if let Ok(q) = QUEUE.lock() {
        let task = fn_task(move |id| {
            let client = reqwest::blocking::Client::new();
            let path = format!(
                "{}/guild/{}/features/persistent_gear/player/{}/loadout",
                *HOST,
                *GUILD.read().unwrap(),
                discord
            );
            info!("[{}] Storing loadout to {}", id, path);
            if let Ok(response) = client
                .put(path)
                .header("Authorization", TOKEN.read().unwrap().clone())
                .header("Content-Type", "application/json")
                .body(loadout.clone())
                .send()
            {
                if response.status().is_success() {
                    info!("[{}] Stored loadout", id);
                    ctx.callback(
                        "dynulo_core",
                        "features:persistent-gear:player:loadout:store",
                        Some(vec!["stored".to_string(), steam.clone()]),
                    );
                } else {
                    error!("[{}] Failed to store loadout", id);
                    ctx.callback(
                        "dynulo_core",
                        "features:persistent-gear:player:loadout:store",
                        Some(vec!["error".to_string(), steam.clone()]),
                    );
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
