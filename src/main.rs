use std::io;

fn shelly_loop() {
    let status: bool;

    loop {
        print!("> ");

        // Read line
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line)
            .expect("Shelly error: Failed to read a line.");

        // Separate strings with spaces.
        let mut arguments: Vec<&str> = input_line.split(' ').collect();
        
        status = execute(arguments);
        if (!status) {
            break;
        }
    }
}
// fn read_line() ->String {
//     let buffer_size: i32 = 1024;
//     let position: i32 = 0;
//     let buffer = Box::new(buffer_size);
//     let mut c: i32 = 0;

//     loop {
//         // Read a character
//         c = getchar();

//         // If we hit EOF, replace it with a null character and return.
//         if (c == )
//     }
// }

fn main() {
    let input: Vec<String> = env::args().collect();
    shelly_loop();
}
