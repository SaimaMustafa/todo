use std::io;
use std::io::Write;

// handling tasks structure
struct Task {
    task: String,
    timestamp_create: u32,
    timestamp_done: u32,
    task_done: bool
}

fn introduction() {
    println!("Simple TODOs management app");
    println!("By amig2004");
    println!("");
     
}


fn main() {
    introduction();
    
    // action choose 
    let mut action_str = String::new();
    
    //main loop
    loop {
        // display possible actions
        println!("[0] - exit app");
        
        // prompt
        print!(">: ");
        io::stdout().flush();
        
        // action selection
        io::stdin()
            .read_line(&mut action_str)
            .expect("Action read failed");

        // parse action value
        let action: u8  = action_str.parse::<u8>().unwrap();

        //execute given action
        // if action equals zero -> exit app
        if action == 0 {
            break
        }



        }

    
}
