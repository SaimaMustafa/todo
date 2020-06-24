use std::io;
use std::io::Write;

// handling tasks structure
#[derive(Clone)]
struct Task {
    task: String,
    timestamp_create: u32,
    timestamp_done: u32,
    task_done: bool
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}


fn display_prompt(prompt: &str) {
    // prompt
    print!("{}", prompt.to_string());
    #[warn(unused_must_use)]
    io::stdout().flush();
}




fn introduction() {
    println!("Simple TODOs management app");
    println!("By amig2004");
    println!("");
     
}

fn create_task(vecref: &mut Vec<Task>){
    // read the new task string 
    display_prompt("[new task]  ");
    let mut task_str = String::new();
    io::stdin()
        .read_line(&mut task_str)
        .expect("Failed to read task action");

    // create new Task object
    let new_task = Task {
        task: task_str,
        timestamp_create: 2645762,
        timestamp_done: 0,
        task_done: false

    };

    // push the object to vector list and clear screen
    vecref.push(new_task);
    clear_screen();    


}


fn delete_task(vecref: &mut Vec<Task>) {
    display_prompt("[Task ID] ");
    
    let mut task_str = String::new();
    io::stdin()
        .read_line(&mut task_str)
        .expect("Failed to read task ID");

    let task_id = task_str.trim().parse::<usize>().unwrap();

    vecref.remove(task_id);
    clear_screen();
}


fn main() {
    clear_screen();
    introduction();
    
    let mut tasks: Vec<Task> = Vec::new();

    //main loop
    loop {

        //TODO load tasks from file
        
        
    
        //display current tasks
        if tasks.len() == 0 {
            println!("INFO: No tasks defined");
        } else {
            let mut cnt = 0;
            let task_list = &mut tasks;
            for task in task_list {
                println!("----------------------------------");
                println!("ID: {}", cnt);
                println!("{}", task.task);
                println!("Status: {}", match task.task_done {
                    false => "Not done",
                    true => "Done"
                });
                println!("Created: {} | Done: {}", task.timestamp_create, task.timestamp_done);
                cnt += 1;
            }
            println!("----------------------------------");
        }

        println!("");
        // display possible actions
        println!("[1] - add task");
        println!("[2] - switch task status");
        println!("");
        println!("[4] - delete task");
        println!("[0] - exit app");
        
        
        // prompt
        display_prompt("[action] ");
        
        // action var
        let mut action = String::new();

        // action selection line read
        io::stdin()
            .read_line(&mut action)
            .expect("Action read failed");

        // parse action value
        let action = match action.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //execute given action
        // if action equals zero -> exit app
        if action == 0 {
            break
        }
        let task_list = &mut tasks;
        match action {
            1 => create_task(task_list),
            2 => println!("This is gonna be second action"),
            4 => delete_task(task_list),
            _ => println!("No action found")

        };


        


    }
    println!("Goodbye.");
    
}
