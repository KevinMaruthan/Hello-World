// To define a function prefix the name of the function with `fn`
// fn function_name(){
//  // program logic
// }

// After each exrepssion

//Start of the program
fn main() {
    let mut name = "james";
    // println!("Hello {name}"); // This is a macro .. Anything ending with `!` is a macro
    //                           // Macros are not function
    //                           //  When a macro is used.. a bunch of code that defines the macro would be replaced at that position.
    //                           //
    //
    print(name);
    let mut name = "kevin";
    print(name);
}

fn print(mut str: &str) {
    str = "sundar";
    println!("Hello {}", str);
}
