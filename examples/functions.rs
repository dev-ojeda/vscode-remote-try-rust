#![allow(unused)]

fn main() {
    println!("Definiendo Funciones!!!");
    another_function();
    function_args(2, &mut String::from("Nelson"));
    println!("result main {}", function_return(2));
}

/* Simple */
fn another_function() {
    println!("another_functions() !!!");
}

/* With arguments */
fn function_args(x: i32, name: &mut String) {
    println!("arg x: {}, arg name: {}", x, name);
    let other_name = name.to_string();
    println!("other {}", other_name);
}


/* With arguments and return */
fn function_return(x: i32) -> i32{
    let result = x + 2;
    println!("result function {}", result);
    result
}