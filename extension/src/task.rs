#[derive(Debug)]
pub enum Task {
    ApiCall {
        id: String,
        method: String,
        path: String,
        body: String,
    },
}

impl Task {
    pub fn apicall(id: String, method: String, path: String, body: String) -> Task {
        Task::ApiCall {
            id,
            method,
            path,
            body,
        }
    }
}
