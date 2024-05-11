
#![allow(unused)]

//Constante Global
const DEFAULT_SPEED: i32 = 500;

fn main() {
    println!("Definiendo variables.rs!!!!");

    //Variables
    let mut x: i32 = 123;
    x = 125;
    println!("VALOR x: {} ", x);

    //Constantes
    const Y: i32 = 123;
    println!("VALOR Y: {} ", Y);
    println!("VALOR DEFAULT_SPEED * x: {} ", DEFAULT_SPEED * x);
    
    //Shadowing
    let c = "123";
    let c_number: i32 = c.parse().unwrap();
    let a = c_number * 2;
    println!("VALOR a: {} ", a);
    

}