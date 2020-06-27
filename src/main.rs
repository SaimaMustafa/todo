use std::io;
use std::io::Write;

// handling tasks structure
#[derive(Clone)]
struct Task {
    task: String,           //Task string
    timestamp_create: u32,  //Creation timestamp    
    timestamp_done: u32,    //Done timestamp
    task_status: bool       // Status (True => Done, False => Not done)
}




fn clear_screen() {
    // Moving console cursor to first 1st row 1st col
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}


fn display_prompt(prompt: &str) {
    // prompt
    print!("{}", prompt.to_string());
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
        task_status: false

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

    //parse task id (String type) into usize
    let task_id = task_str.trim().parse::<usize>().unwrap();
    
    // verify the vector length to prevent thread panic
    if task_id <= (vecref.len()-1) {
        vecref.remove(task_id);
        clear_screen();
    }
    else {
        clear_screen();
    }
}

fn switch_task_status(vecref: &mut Vec<Task>) {
    display_prompt("[Task ID] ");
    
    let mut task_str = String::new();
    io::stdin()
        .read_line(&mut task_str)
        .expect("Failed to read task ID");

    let task_id = task_str.trim().parse::<usize>().unwrap();
    
    // get current task status
    if task_id <= (vecref.len()-1) {
        // verify if the given index does not exceeds the vector length to avoid thread panic
        let current_status: bool = vecref[task_id].task_status; 
    
        vecref[task_id].task_status = !current_status;
        clear_screen();
    } 
    else {
        clear_screen();
    }
    
}

// fn loadTasks(vecref: &mut Vec<Task>) {
//     // load up text file containing the tasks
//     let tasks_file = fs::File::open("tasks");

//         // if the files does not exist - create it
//     // iterate over the file to load up the tasks
//     let data = fs::read_to_string(tasks_file).expect("Problem appeared during file reading");
// }


fn main() {
    //TODO load tasks from text file
    let mut tasks: Vec<Task> = Vec::new();

    // loadTasks(&mut tasks);


    //main loop
    loop {
        clear_screen();
        introduction();
        //TODO load tasks from file
    
        //display current tasks
        if tasks.len() == 0 {
            println!("INFO: No tasks defined");
        } else {
            let mut cnt = 0;
            let task_list = &mut tasks;
            
            println!("Current tasks: ");
            for task in task_list {
                println!("----------------------------------");
                println!("ID: {}", cnt);
                println!("{}", task.task);
                println!("Status: {}", match task.task_status {
                    false => "Not done",
                    true => "Done"
                });
                println!("Created: {} | Done: {}", task.timestamp_create, task.timestamp_done);
                cnt += 1;
            }
            
        }

        println!("----------------------------------");
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
            2 => switch_task_status(task_list),
            4 => delete_task(task_list),
            _ => println!("No action found")

        };


        


    }
    println!("Goodbye.");
    
}
