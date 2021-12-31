use arma_rs::{Group, Context};

use crate::{
    commands::core::{GUILD, HOST, TOKEN},
    worker::fn_task,
    QUEUE,
};

pub fn group() -> Group {
    Group::new().command("call", call)
}

fn call(ctx: Context, method: String, path: String, body: String) -> String {
    if let Ok(q) = QUEUE.lock() {
        let task = fn_task(move |id| {
            let client = reqwest::blocking::Client::new();
            let path = format!("{}/guild/{}{}", *HOST, GUILD.read().unwrap(), path);
            info!("[{}] {} {}", id, method, path);
            let builder = match method.to_lowercase().as_str() {
                "get" => client.get(&path),
                "post" => client.post(&path),
                "put" => client.put(&path),
                "delete" => client.delete(&path),
                _ => {
                    error!("Unsupported method: {}", method);
                    return Ok(());
                }
            };
            if let Ok(response) = builder
                .header("Authorization", TOKEN.read().unwrap().clone())
                .body(body.clone())
                .send()
            {
                if let Ok(body) = response.text() {
                    ctx.callback("dynulo_core", "api:call", Some(vec![id, body]));
                } else {
                    warn!("[{}] failed to fetch {}", id, path);
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
