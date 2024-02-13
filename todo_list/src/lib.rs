use std::sync::atomic::{self, AtomicU64};

#[derive(Debug)]
pub struct Task{
    task: String,
    done_status: bool, 
    id: u64, 
}

pub fn run(args: Vec<&str>, todo: &mut Vec<Task>){

    parse_arguments(args, todo); 
        
}

pub fn parse_arguments(args:Vec<&str>,todo_list:&mut Vec<Task>){
    let command = args[0];

    match command{
        "add" => {
            if let Some(value) = args.get(1){
                let new_task = *value;
                add_new_task(todo_list,*value);
                display_todo(todo_list);
            }else{
                println!("Please provide a new name for the task");
            }
        },
        "show" => {
            display_todo(todo_list)
        },
        "delete" => {
            match &args[1].parse::<u64>() {
                Ok(value) => {
                    remove_task(todo_list,*value)
                }
                Err(message)=>{
                    println!("{}",message.to_string());
                }
            }
        },
        "update" => {
            match &args[1].parse::<u64>() {
                Ok(value) =>{
                    
                }
                Err(message )=> {
                    print!("{}",message);
                }
            }
        }
        "help" | _ => {

        }
    }
}

static UNIQUE_ID:AtomicU64 = AtomicU64::new(1);

pub fn add_new_task(todo_list:&mut Vec<Task>,task_string:&str){
    let id_no = UNIQUE_ID.fetch_add(1, atomic::Ordering::SeqCst);
    let task :Task = Task{
        task:task_string.into(),
        done_status:false,
        id:id_no,
    };

    todo_list.push(task);
    print!("{} added to the todo list:",task_string);
}

fn display_todo(todo_list:&Vec<Task>){
    if todo_list.len()<1{
        println!("Empty todo list");
        return;
    }

    for item in todo_list{
        println!("id:{},name{},done{}",item.id,item.task,item.done_status)
    }

}

fn remove_task(todo_list:&mut Vec<Task>,id_no:u64){
    todo_list.retain(|task|task.id != id_no)
}