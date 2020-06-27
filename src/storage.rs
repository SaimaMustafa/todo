
/*
    todo_storage - main structure of Taskbase object and all it's methods

    taskbase default location is ~/.todo_taskbase

    file content strcute:
        task_string_no.1|created_timestamp|done_timestamp|status
        task_string_no.2|created_timestamp|done_timestamp|status
        task_string_no.3|created_timestamp|done_timestamp|status
        task_string_no.4|created_timestamp|done_timestamp|status
        ...

    "|" -> separator char
*/  

use std::fs;
use std::timestamp;

mod todo_storage {

    const default_path = "~";
    const default_storage_file = ".todo_taskbase";
    const backup_storage_file = ".todo_taskbase.backup";

    // Structure for representing task-storing files
    struct Taskbase {
        tasks: String               // Tasks vector list
        updated: u32                // Last update timestamp
    };

    // fn refresh_updated(&mut tb_ref: Taskbase) {
    //     // refresh updated timestamp

    //     tb_ref.updated
    // }


    fn loadBase() -> Taskbase {
        // open storage file and load it's content
        
        // load file out of ~/.todo_taskbase and prepare content string
        let filepath = default_path+default_storage_file;
        let storage_file = File::open(filepath)?;
        let storage_file_metadata = fs::metadata(fielpath)?;
        
        let content = String::new();

        // load content from file 
        storage_file.read_to_string(content);

        // return loaded file as object
        Taskbase{
            tasks = content,
            updated = storage_file_metadata.modified()
        }

    }

    // save current database to file
    fn saveBase(& tb_ref: Taskbase) -> std::io::Result<()> {
        // prepare filepaths for writing
        let filepath = default_path+default_storage_file;
        let backup_filepath = filepath + backup_storage_file;

        // prepare backup
        fs::copy(filepath, backup_filepath)?;
        
        // save file
        let storage_file = File::open(filepath)?;
        storage_file.write

        // if everything went smooth -> delete backup
        
        Ok(())
    }



}

