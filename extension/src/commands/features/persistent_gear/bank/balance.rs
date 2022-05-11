use arma_rs::{Context, Group, IntoArma};
use reqwest::StatusCode;

use crate::{
    commands::core::{GUILD, HOST, TOKEN},
    worker::fn_task,
    QUEUE,
};

pub fn group() -> Group {
    Group::new().command("fetch", fetch)
}

fn fetch(ctx: Context, discord: String, steam: String) -> String {
    if let Ok(q) = QUEUE.lock() {
        let task = fn_task(move |id| {
            let steam = steam.clone();
            let client = reqwest::blocking::Client::new();
            let path = format!(
                "{}/guild/{}/features/persistent_gear/player/{}/bank/balance",
                *HOST,
                *GUILD.read().unwrap(),
                discord
            );
            info!("[{}] fetching balance from {}", id, path);
            if let Ok(response) = client
                .get(path)
                .header("Authorization", TOKEN.read().unwrap().clone())
                .header("Content-Type", "application/json")
                .send()
            {
                match response.status() {
                    StatusCode::OK => match response.text() {
                        Ok(text) => {
                            info!("[{}] fetched balance", id);
                            if let Ok(balance) = text.parse::<i32>() {
                                ctx.callback(
                                    "dynulo_core",
                                    "features:persistent-gear:bank:balance:fetch",
                                    Some(vec![
                                        "loaded".to_arma(),
                                        steam.to_arma(),
                                        balance.to_arma(),
                                    ]),
                                );
                            } else {
                                error!("[{}] failed to convert balance: {}", id, text);
                                ctx.callback(
                                    "dynulo_core",
                                    "features:persistent-gear:bank:balance:fetch",
                                    Some(vec!["error".to_string(), steam]),
                                );
                            }
                        }
                        Err(e) => {
                            error!("[{}] failed to fetch balance: {}", id, e);
                            ctx.callback(
                                "dynulo_core",
                                "features:persistent-gear:bank:balance:fetch",
                                Some(vec!["error".to_string(), steam]),
                            );
                        }
                    },
                    StatusCode::NO_CONTENT => {
                        info!("[{}] no balance", id);
                        ctx.callback(
                            "dynulo_core",
                            "features:persistent-gear:bank:balance:fetch",
                            Some(vec!["loaded".to_arma(), steam.to_arma(), 0.to_arma()]),
                        );
                    }
                    s => {
                        error!("[{}] failed to fetch balance: {}", id, s);
                        ctx.callback(
                            "dynulo_core",
                            "features:persistent-gear:bank:balance:fetch",
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
