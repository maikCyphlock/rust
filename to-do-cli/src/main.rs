use std::io;

struct TodoItem {
    id: u64,
    title: String,
    completed: bool,
}

struct TodoList {
    items: Vec<TodoItem>,
}
impl TodoList {
    fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    fn add_item(&mut self, title: String) {
        let id = self.items.len() as u64 + 1;
        let new_item = TodoItem {
            id,
            title: title.clone(),
            completed: false,
        };
        self.items.push(new_item);
        println!("Added item: {}", title);
    }
    fn list_items(&self) {
        if self.items.is_empty() {
            println!("No items found");
        } else {
            for item in &self.items {
                println!("{}: {}", item.id, item.title);
            }
        }
    }

    fn complete_item(&mut self, id: u64) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = true;
            println!("Item {} completed", id);
        } else {
            println!("Item {} not found", id);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    loop {
        println!("-------Enter your choice:");
        println!("-------1. Add item");
        println!("-------2. List items");
        println!("-------3. Complete item");
        println!("-------4. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                continue;
            }
        };
        match choice {
            1 => {
                println!("Enter the title of the new item:");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line");
                todo_list.add_item(title.trim().to_string());
            }
            2 => {
                todo_list.list_items();
            }
            3 => {
                println!("Enter the ID of the completed item:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u64 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                todo_list.complete_item(id);
            }
            4 => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 4.");
            }
        }
    }
}
