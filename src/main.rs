use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;
use cli_table::{format::Justify, print_stdout, Style, Table, WithTitle, Color, CellStruct};

use serde::{Deserialize, Serialize};

const STORAGE_PATH_DIR: &str = "D:\\Projects\\Rust\\Storage\\todo.json";

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
enum Status {
    PENDING,
    COMPLETED,
}




impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::COMPLETED => write!(f, "COMPLETED"),
            Status::PENDING => write!(f, "PENDING")
        }
    }
}

fn custom_cell(cell: CellStruct, status: &Status ) -> CellStruct{
    match status {
        Status::PENDING => cell.foreground_color(Option::from(Color::Red)),
        Status::COMPLETED => cell.foreground_color(Option::from(Color::Green)),
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Table)]
struct Todo {

    #[table(title = "Id" , justify = "Justify::Left",color = "Color::Yellow")]
    id: usize,
    #[table(title = "Title" , justify = "Justify::Left",color = "Color::Cyan")]
    title: String,
    #[table(title = "Status", justify = "Justify::Left", customize_fn="custom_cell")]
    status: Status,
}

struct TodoStorage {
    storage: Vec<Todo>,
}

impl TodoStorage {
    fn new() -> TodoStorage {
        TodoStorage {
            storage: Vec::<Todo>::new()
        }
    }

    fn load_todos_from_storage_dir(&mut self) {
        let path = Path::new(STORAGE_PATH_DIR);
        if path.exists() {
            let todo_stream = fs::read_to_string(STORAGE_PATH_DIR).unwrap();
            let mut todo_list: Vec<Todo> = serde_json::from_str(&todo_stream).unwrap();
            self.storage.append(&mut todo_list);
        }
    }

    fn save_todo_state_into_storage_dir(&mut self) {
        let todo_state = serde_json::to_string(&self.storage).unwrap();
        fs::write(STORAGE_PATH_DIR, todo_state).unwrap();
    }


    fn print_all_todos(&mut self) {
        for (index,todo) in self.storage.iter_mut().enumerate(){
            (*todo).id = index+1;
        }
        print_stdout(self.storage.with_title()).unwrap();
    }


    fn store(&mut self, todo: Todo) {
        self.storage.push(todo);
    }
}


impl Todo {
    pub fn new(title: &str) -> Todo {
        return Todo {
            id: 0,
            title: title.to_string(),
            status: Status::PENDING,
        };
    }
}


fn create_todo(todo_storage: &mut TodoStorage, title: &str) {
    let mut todo = Todo::new(title);
    todo.id = todo_storage.storage.len()+1;
    todo_storage.store(todo);
    get_all_todos(todo_storage);
}


fn get_all_todos(todo_storage: &mut TodoStorage) {
    todo_storage.print_all_todos();
}


fn update_todo(todo_storage: &mut TodoStorage, id: usize) {

    if id> todo_storage.storage.len() {
        get_all_todos(todo_storage);
        panic!("Enter a valid id");
    }

    let todo = &mut todo_storage.storage[id-1];


    println!("{:#?}", todo);
    todo.status = Status::COMPLETED;

    get_all_todos(todo_storage);
}

fn delete_todo(todo_storage: &mut TodoStorage, id: usize){
    if id> todo_storage.storage.len() {
        get_all_todos(todo_storage);
        panic!("Enter a valid id");
    }
    todo_storage.storage.retain(|todo| todo.id != id);
    get_all_todos(todo_storage);
}


fn process_args(args: &Vec<String>, todo_storage: &mut TodoStorage) -> bool {
    let mut ans = true;
    if args.len() > 4 {
        return false;
    }
    let len = args.len();
    match args[1].as_str() {
        "--all" if len == 2 => get_all_todos(todo_storage),
        "-a" if len == 3 => create_todo(todo_storage, args[2].as_str()),
        "-u" if len == 4 => {
            match args[3].as_str() {
                "-c" => update_todo(todo_storage, args[2].parse::<usize>().unwrap()),
                _ => ans = false,
            }
        }
        "-d" => { delete_todo(todo_storage,args[2].parse::<usize>().unwrap()) }
        _ => { ans = false }
    }
    return ans;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut storage: TodoStorage = TodoStorage::new();

    // Load the todos from FS
    storage.load_todos_from_storage_dir();
    let result = process_args(&args, &mut storage);
    storage.save_todo_state_into_storage_dir();

    if !result {
        println!("The operation was not success please give proper arguments according to the documentation");
    }
    // println!("{:?}", args);
}
