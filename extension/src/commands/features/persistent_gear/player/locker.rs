use arma_rs::{Context, Group, IntoArma, Value};
use serde::{Deserialize, Serialize};

use crate::{
    commands::core::{GUILD, HOST, TOKEN},
    worker::fn_task,
    QUEUE,
};

pub fn group() -> Group {
    Group::new()
        .command("fetch", fetch)
        .command("store", store)
        .command("take", take)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "s")]
    pub class: String,
    #[serde(rename = "q")]
    pub quantity: i32,
}

impl IntoArma for Item {
    fn to_arma(&self) -> Value {
        vec![self.class.to_arma(), self.quantity.to_arma()].to_arma()
    }
}

fn fetch(ctx: Context, discord: String, steam: String) -> String {
    if let Ok(q) = QUEUE.lock() {
        let task = fn_task(move |id| {
            let steam = steam.clone();
            let client = reqwest::blocking::Client::new();
            let path = format!(
                "{}/guild/{}/features/persistent_gear/player/{}/locker",
                *HOST,
                *GUILD.read().unwrap(),
                discord
            );
            info!("[{}] fetching locker from {}", id, path);
            if let Ok(response) = client
                .get(path)
                .header("Authorization", TOKEN.read().unwrap().clone())
                .header("Content-Type", "application/json")
                .send()
            {
                if let Ok(json) = response.json::<Vec<Item>>() {
                    info!("[{}] fetched {} locker", id, json.len());
                    ctx.callback(
                        "dynulo_core",
                        "features:persistent-gear:player:locker:fetch",
                        Some(vec!["clear".to_string(), steam.clone()]),
                    );
                    for item in json {
                        ctx.callback(
                            "dynulo_core",
                            "features:persistent-gear:player:locker:fetch",
                            Some(vec!["entry".to_arma(), steam.to_arma(), item.to_arma()]),
                        );
                    }
                    ctx.callback(
                        "dynulo_core",
                        "features:persistent-gear:player:locker:fetch",
                        Some(vec!["done".to_string(), steam]),
                    );
                } else {
                    warn!("[{}] failed to fetch locker", id);
                    ctx.callback(
                        "dynulo_core",
                        "features:persistent-gear:player:locker:fetch",
                        Some(vec!["error".to_string(), steam]),
                    );
                }
            } else {
                warn!("[{}] failed to fetch locker", id);
                ctx.callback(
                    "dynulo_core",
                    "features:persistent-gear:player:locker:fetch",
                    Some(vec!["error".to_string(), steam]),
                );
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

fn store(ctx: Context, discord: String, steam: String, items: Vec<(String, i32)>) -> String {
    if let Ok(q) = QUEUE.lock() {
        let task = fn_task(move |id| {
            let client = reqwest::blocking::Client::new();
            let path = format!(
                "{}/guild/{}/features/persistent_gear/player/{}/locker/store",
                *HOST,
                *GUILD.read().unwrap(),
                discord
            );
            info!("[{}] Storing locker to {}", id, path);
            if let Ok(response) = client
                .put(path)
                .header("Authorization", TOKEN.read().unwrap().clone())
                .header("Content-Type", "application/json")
                .json(
                    &items
                        .clone()
                        .into_iter()
                        .map(|(class, quantity)| Item { class, quantity })
                        .collect::<Vec<Item>>(),
                )
                .send()
            {
                if response.status().is_success() {
                    info!("[{}] Stored locker", id);
                    ctx.callback(
                        "dynulo_core",
                        "features:persistent-gear:player:locker:store",
                        Some(vec!["stored".to_string(), steam.clone()]),
                    );
                } else {
                    error!("[{}] Failed to store locker", id);
                    ctx.callback(
                        "dynulo_core",
                        "features:persistent-gear:player:locker:store",
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

fn take(ctx: Context, discord: String, steam: String, items: Vec<(String, i32)>) -> String {
    if let Ok(q) = QUEUE.lock() {
        let task = fn_task(move |id| {
            let client = reqwest::blocking::Client::new();
            let path = format!(
                "{}/guild/{}/features/persistent_gear/player/{}/locker/take",
                *HOST,
                *GUILD.read().unwrap(),
                discord
            );
            info!("[{}] Taking locker from {}", id, path);
            if let Ok(response) = client
                .put(path)
                .header("Authorization", TOKEN.read().unwrap().clone())
                .header("Content-Type", "application/json")
                .json(
                    &items
                        .clone()
                        .into_iter()
                        .map(|(class, quantity)| Item { class, quantity })
                        .collect::<Vec<Item>>(),
                )
                .send()
            {
                if response.status().is_success() {
                    info!("[{}] took locker", id);
                    ctx.callback(
                        "dynulo_core",
                        "features:persistent-gear:player:locker:take",
                        Some(vec!["stored".to_string(), steam.clone()]),
                    );
                } else {
                    error!("[{}] Failed to take locker", id);
                    ctx.callback(
                        "dynulo_core",
                        "features:persistent-gear:player:locker:take",
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
