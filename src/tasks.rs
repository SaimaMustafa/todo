
mod todo_tasks{
    struct Task {
        task: String,           //Task string
        timestamp_create: u32,  //Creation timestamp    
        timestamp_done: u32,    //Done timestamp
        task_status: bool       // Status (True => Done, False => Not done)
    }
}