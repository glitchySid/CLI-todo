pub mod data_mainpulation{
    use rusqlite::{params, Connection, Result};
    use std::io::{Write, stdout};

    #[derive(Debug)]
    pub struct Task{
        pub id: i32,
        pub title: String,
        pub priority: i8,
    }

    pub fn create_database(connection: &Connection) {
        let _ = connection.execute("CREATE TABLE todotable (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            priority INTEGER)", params![]);
    }

    pub fn add_task(connection: &Connection, task: Task) -> Result<()>{
        connection.execute(
            "INSERT INTO todotable (title, priority) VALUES (?1, ?2)",
            (task.title, task.priority),
        )?;
        Ok(())
    }

    pub fn get_data(connection: &Connection) -> Result<Vec<Task>>{
       let mut statement = connection.prepare("SELECT id, title, priority FROM todotable")?;
        let tasks = statement.query_map([], |row| {
            Ok(Task{
                id: row.get(0)?,
                title: row.get(1)?,
                priority: row.get(2)?,
            })
        })?.collect::<Result<Vec<_>, _>>();
        return tasks;

    }
    pub fn delete_query(connection: &Connection, id: i32){
        let _ = connection.execute("DELETE FROM todotable WHERE id=?", [id]);
    }
    pub fn display_tasks_table(tasks: &[Task]) {
        if tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        let id_width = tasks.iter().map(|t| t.id.to_string().len()).max().unwrap_or(2).max(2);
        let title_width = tasks.iter().map(|t| t.title.len()).max().unwrap_or(5).max(5);
        let priority_width = tasks.iter().map(|t| t.priority.to_string().len()).max().unwrap_or(8).max(8);

        let total_width = id_width + title_width + priority_width + 10; // 10 for borders and spaces

        println!("┏{}┓", "━".repeat(total_width - 2));
        println!("┃ {:^id_width$} │ {:^title_width$} │ {:^priority_width$} ┃", 
                 "ID", "Title", "Priority",
                 id_width = id_width,
                 title_width = title_width,
                 priority_width = priority_width);
        println!("┣{}┫", "━".repeat(total_width - 2));

        for task in tasks {
            println!("┃ {:>id_width$} │ {:<title_width$} │ {:>priority_width$} ┃", 
                     task.id,
                     task.title,
                     task.priority,
                     id_width = id_width,
                     title_width = title_width,
                     priority_width = priority_width);
        }

        println!("┗{}┛", "━".repeat(total_width - 2));
        stdout().flush().unwrap();
    }
}
