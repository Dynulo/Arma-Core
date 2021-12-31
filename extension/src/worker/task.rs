use uuid::Uuid;

pub struct Task {
    id: String,
    handler: Box<dyn Fn(String) -> Result<(), String> + Send>,
}

impl Task {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn execute(&self) -> Result<(), String> {
        (self.handler)(self.id.clone())
    }
}

pub fn fn_task<T>(exe: T) -> Task
where
    T: TaskExecutor + 'static + Send,
{
    Task {
        id: Uuid::new_v4().to_string(),
        handler: Box::new(move |id| exe.execute(id)),
    }
}

pub trait TaskExecutor {
    fn execute(&self, id: String) -> Result<(), String>;
}

impl<F> TaskExecutor for F
where
    F: Fn(String) -> Result<(), String> + Send,
{
    fn execute(&self, id: String) -> Result<(), String> {
        self(id)
    }
}
