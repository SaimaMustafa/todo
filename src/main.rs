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
    
    
    
    //main loop
    loop {

        // display possible actions
        println!("[1] - first action");
        println!("[0] - exit app");
        
        // prompt
        print!(">: ");
        #[warn(unused_must_use)]
        io::stdout().flush();
        
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

        match action {
            1 => println!("This is gonna be first action"),
            2 => println!("This is gonna be second action"),
            _ => println!("No action found")

        }


    }
    println!("Goodbye.")
    
}
