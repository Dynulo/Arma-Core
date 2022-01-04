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
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: i64,
    pub position: i64,
}

impl IntoArma for Role {
    fn to_arma(&self) -> Value {
        vec![
            self.id.to_string(),
            self.name.to_string(),
            self.color.to_string(),
            self.position.to_string(),
        ]
        .to_arma()
    }
}

fn fetch(ctx: Context) -> String {
    if let Ok(q) = QUEUE.lock() {
        let task = fn_task(move |id| {
            let client = reqwest::blocking::Client::new();
            let path = format!("{}/guild/{}/roles", *HOST, GUILD.read().unwrap());
            info!("[{}] fetching roles from {}", id, path);
            if let Ok(response) = client
                .get(path)
                .header("Authorization", TOKEN.read().unwrap().clone())
                .header("Content-Type", "application/json")
                .send()
            {
                if let Ok(json) = response.json::<Vec<Role>>() {
                    info!("[{}] fetched {} roles", id, json.len());
                    ctx.callback("dynulo_core", "discord:roles:fetch", Some(vec!["clear"]));
                    for role in json {
                        ctx.callback(
                            "dynulo_core",
                            "discord:roles:fetch",
                            Some(vec!["entry".to_arma(), role.to_arma()]),
                        );
                    }
                    ctx.callback("dynulo_core", "discord:roles:fetch", Some(vec!["done"]));
                } else {
                    warn!("[{}] failed to fetch roles", id);
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
