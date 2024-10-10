pub mod tui_input{
    use std::io::{stdin, stdout, Write, Read};
    pub fn name_input() -> String{
        let box_width = 43;
        let placeholder = " ".repeat(box_width - 4);  // -4 for "┃ " and " ┃"
        
        // Draw the complete box
        println!("┏{}┓", "━".repeat(box_width - 2));
        println!("┃ {}┃", placeholder);
        println!("┗{}┛", "━".repeat(box_width - 2));

        // Move cursor up two lines
        print!("\x1B[2A");
        stdout().flush().expect("Failed to flush stdout");

        // Print prompt and get input
        print!("\r┃ Enter the Task: ");
        stdout().flush().expect("Failed to flush stdout");

        let mut input_string = String::new();
        stdin().read_line(&mut input_string).expect("error reading input");

        // Move cursor down two lines
        print!("\x1B[2B");
        stdout().flush().expect("Failed to flush stdout");

        let name = input_string.trim();
        name.to_string()
    }
    pub fn priority_input() -> i8{
        
        let box_width = 43;
        let placeholder = " ".repeat(box_width - 4);  // -4 for "┃ " and " ┃"
        
        // Draw the complete box
        println!("┏{}┓", "━".repeat(box_width - 2));
        println!("┃ {}┃", placeholder);
        println!("┗{}┛", "━".repeat(box_width - 2));

        // Move cursor up two lines
        print!("\x1B[2A");
        stdout().flush().expect("Failed to flush stdout");

        // Print prompt and get input
        print!("\r┃ Enter 1 to 10: ");
        stdout().flush().expect("Failed to flush stdout");

        let mut input_string = String::new();
        stdin().read_line(&mut input_string).expect("error reading input");

        // Move cursor down two lines
        print!("\x1B[2B");
        stdout().flush().expect("Failed to flush stdout");

        let float_value: i8 = input_string.trim().parse().expect("error converting into float_value");
        float_value
    }
    pub fn wdwtd() -> u8 {
        println!("What do you Want to do? \n 1. Add a task \n 2. Delete a Task \n 3. View the todo-list \n 4. Quit \n");
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).expect("Error reading message");
        let commands: u8 = input_string.trim().parse().expect("problem in parsing the string");
        return commands;    


    }
    pub fn delte_input() -> i32{
        
        let box_width = 43;
        let placeholder = " ".repeat(box_width - 4);  // -4 for "┃ " and " ┃"
        
        // Draw the complete box
        println!("┏{}┓", "━".repeat(box_width - 2));
        println!("┃ {}┃", placeholder);
        println!("┗{}┛", "━".repeat(box_width - 2));

        // Move cursor up two lines
        print!("\x1B[2A");
        stdout().flush().expect("Failed to flush stdout");

        // Print prompt and get input
        print!("\r┃ Enter the Task id: ");
        stdout().flush().expect("Failed to flush stdout");

        let mut input_string = String::new();
        stdin().read_line(&mut input_string).expect("error reading input");

        // Move cursor down two lines
        print!("\x1B[2B");
        stdout().flush().expect("Failed to flush stdout");

        let name: i32 = input_string.trim().parse().expect("Error converting to integer");
        name
    }
    pub fn press_any_key_to_continue() {
        println!("Press any key to continue...");
        let _ = stdin().read(&mut [0u8]).unwrap();
    }
}
