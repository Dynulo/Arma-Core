use arma_rs::{Context, Group, IntoArma, Value};
use serde::Deserialize;

use crate::{
    commands::core::{GUILD, HOST, TOKEN},
    worker::fn_task,
    QUEUE,
};

pub fn group() -> Group {
    Group::new().command("fetch", fetch)
}

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(rename = "s")]
    pub class: String,
    // #[serde(rename = "p")]
    // pub pretty: String,
    #[serde(rename = "c")]
    pub cost: i32,
    #[serde(rename = "r")]
    pub roles: Option<String>,
    // #[serde(rename = "t")]
    // pub category: Option<String>,
    #[serde(rename = "g")]
    pub global: bool,
}

impl IntoArma for Item {
    fn to_arma(&self) -> Value {
        vec![
            self.class.to_arma(),
            // self.pretty.to_arma(),
            self.cost.to_arma(),
            {
                let roles = self.roles.clone().unwrap_or_default();
                if roles.is_empty() {
                    Value::Array(vec![])
                } else {
                    roles
                        .split('|')
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .to_arma()
                }
            },
            // self.category.to_arma(),
            self.global.to_arma(),
        ]
        .to_arma()
    }
}

fn fetch(ctx: Context) -> String {
    if let Ok(q) = QUEUE.lock() {
        let task = fn_task(move |id| {
            let client = reqwest::blocking::Client::new();
            let path = format!(
                "{}/guild/{}/features/persistent_gear/shop/items",
                *HOST,
                *GUILD.read().unwrap(),
            );
            info!("[{}] fetching items from {}", id, path);
            if let Ok(response) = client
                .get(path)
                .header("Authorization", TOKEN.read().unwrap().clone())
                .header("Content-Type", "application/json")
                .send()
            {
                if let Ok(json) = response.json::<Vec<Item>>() {
                    info!("[{}] fetched {} items", id, json.len());
                    ctx.callback(
                        "dynulo_core",
                        "features:persistent-gear:shop:items:fetch",
                        Some(vec!["clear"]),
                    );
                    for item in json {
                        ctx.callback(
                            "dynulo_core",
                            "features:persistent-gear:shop:items:fetch",
                            Some(vec!["entry".to_arma(), item.to_arma()]),
                        );
                    }
                    ctx.callback(
                        "dynulo_core",
                        "features:persistent-gear:shop:items:fetch",
                        Some(vec!["done"]),
                    );
                } else {
                    warn!("[{}] failed to fetch items", id);
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
