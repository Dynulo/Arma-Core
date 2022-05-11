use arma_rs::{Context, Group};
use serde::Serialize;

use crate::{
    commands::core::{GUILD, HOST, TOKEN},
    worker::fn_task,
    QUEUE,
};

pub fn group() -> Group {
    Group::new().command("create", create)
}

#[derive(Debug, Serialize)]
pub struct NewPurchase {
    #[serde(rename = "s")]
    pub class: String,
    #[serde(rename = "q")]
    pub quantity: i32,
    #[serde(rename = "g")]
    pub global: bool,
}

fn create(
    ctx: Context,
    discord: String,
    steam: String,
    items: Vec<(String, i32, i32, i32, bool)>,
) -> String {
    if let Ok(q) = QUEUE.lock() {
        let task = fn_task(move |id| {
            let client = reqwest::blocking::Client::new();
            let path = format!(
                "{}/guild/{}/features/persistent_gear/player/{}/bank/purchases",
                *HOST,
                *GUILD.read().unwrap(),
                discord
            );
            info!("[{}] Creating purchase at {}", id, path);
            if let Ok(response) = client
                .post(path)
                .header("Authorization", TOKEN.read().unwrap().clone())
                .header("Content-Type", "application/json")
                .json(
                    &items
                        .clone()
                        .into_iter()
                        .map(|p| NewPurchase {
                            class: p.0,
                            quantity: p.2,
                            global: p.4,
                        })
                        .collect::<Vec<NewPurchase>>(),
                )
                .send()
            {
                if response.status().is_success() {
                    info!("[{}] Created purchase", id);
                    ctx.callback(
                        "dynulo_core",
                        "features:persistent-gear:bank:purchases:create",
                        Some(vec!["created".to_string(), steam.clone()]),
                    );
                } else {
                    error!("[{}] Failed to create purchase", id);
                    ctx.callback(
                        "dynulo_core",
                        "features:persistent-gear:bank:purchases:create",
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
