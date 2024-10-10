use crate::database::data_mainpulation::*;
use crate::input::tui_input::*;
use rusqlite::Connection;
use std::process::Command;
pub fn run(conn: &Connection) {
    create_database(&conn);
    loop {
        Command::new("clear").status().unwrap();
        let what_to_do = wdwtd();
        if what_to_do == 1 {
            println!("Enter the name of the Task");
            let title_todo: String = name_input();
            println!("Enter the priority value (1-10)");
            let priority_todo = priority_input();
            let todo = Task {
                id: 0,
                title: title_todo,
                priority: priority_todo,
            };
            let _ = add_task(&conn, todo);
            Command::new("clear").status().unwrap();
        } else if what_to_do == 2 {
            let data = get_data(&conn).unwrap_or_else(|_| Vec::new());
            display_tasks_table(&data);
            let id: i32 = delte_input();
            delete_query(&conn, id);
            Command::new("clear").status().unwrap();
        } else if what_to_do == 3 {
            let data = get_data(&conn).unwrap_or_else(|_| Vec::new());
            display_tasks_table(&data);
            press_any_key_to_continue();
            Command::new("clear").status().unwrap();
        } else if what_to_do == 4 {
            break;
        } else {
            println!("Error you entered wrong value!!!");
            Command::new("clear").status().unwrap();
        }
    }
}
