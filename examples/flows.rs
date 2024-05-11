#![allow(unused)]

fn main() {
    let nummber = 4;

    /* if/else */
    let message = if nummber < 5 {
        "Pretty Small" 
    } else if nummber < 100 {
        "Pretty Normal" 
    } else {
        "Pretty Big"
    };

    println!("Mensaje: {} ", message);

    /* for */
    let a = [0,10,20,30,40];
    for item in a.iter() {
        println!("Item: {} ", item);
        if item == &20 {
            break;
        }
    }
}