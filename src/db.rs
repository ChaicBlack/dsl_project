pub(crate) struct Db {
    shared: Arc<Shared>,
}

struct Shared {
    state: Mutex<String, Entry>,

    background_task: Notify,
}
