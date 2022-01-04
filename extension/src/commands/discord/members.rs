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
pub struct Member {
    pub nickname: String,
    pub discord: String,
    pub avatar: String,
    pub roles: Vec<String>,
    pub steam: String,
}

impl IntoArma for Member {
    fn to_arma(&self) -> Value {
        vec![
            self.nickname.to_arma(),
            self.discord.to_arma(),
            self.avatar.to_arma(),
            self.roles.to_arma(),
            self.steam.to_arma(),
        ]
        .to_arma()
    }
}

fn fetch(ctx: Context) -> String {
    if let Ok(q) = QUEUE.lock() {
        let task = fn_task(move |id| {
            let client = reqwest::blocking::Client::new();
            let path = format!("{}/guild/{}/members", *HOST, GUILD.read().unwrap());
            info!("[{}] fetching members from {}", id, path);
            if let Ok(response) = client
                .get(path)
                .header("Authorization", TOKEN.read().unwrap().clone())
                .header("Content-Type", "application/json")
                .send()
            {
                if let Ok(json) = response.json::<Vec<Member>>() {
                    info!("[{}] fetched {} members", id, json.len());
                    ctx.callback("dynulo_core", "discord:members:fetch", Some(vec!["clear"]));
                    for member in json {
                        ctx.callback(
                            "dynulo_core",
                            "discord:members:fetch",
                            Some(vec!["entry".to_arma(), member.to_arma()]),
                        );
                    }
                    ctx.callback("dynulo_core", "discord:members:fetch", Some(vec!["done"]));
                } else {
                    warn!("[{}] failed to fetch members", id);
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
