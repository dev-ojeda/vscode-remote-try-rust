#![allow(unused)]

use std::str::Chars;

fn main() {
    println!("Definiendo Tipos!!!");
    
    /* 
        Integer (scalar type) -> i32
        Integer (scalar type) -> i64
    */
    let x: i32 = 2147483647; //MAX
    let y: i64 = 9223372036854775807; //MAX
    println!("VALOR x: {} ", x);
    println!("VALOR y: {} ", y);

    /*
        Float (scalar type) -> i32
        Float (scalar type) -> i64 
    */

    let f: f32 = 32.5;
    println!("VALOR f: {} ", f);

    /* Operations */
    let z = x as f32 / f;
    println!("VALOR f: {} ", z);

    /* Char */
    let s = String::from("love: ❤️");
    let v: Vec<char> = s.chars().collect();
    let c: char = '😂';
    // for elem in v.iter() {
    //     println!("VALOR emotes: {} ", elem);        
    // }
    println!("VALOR c: {} ", c);        

    /* Tuples */
    let t_num = (1,2,3);
    let t_str = ("1","2","3");
    let t_num_str = ("1",2,"3",4.5);
    println!("VALOR t_num: {:?} ", t_num);
    println!("VALOR t_str: {:?} ", t_str);        
    println!("VALOR t_num_str: {:?} ", t_num_str);
    println!("VALOR tuple index: {} ", t_num_str.3);        

    /* Array */    

    let a = [1,2,3,4];
    println!("VALOR array: {:?} ", a);
    println!("VALOR array index: {} ", a[1]);    

    let s = ["Hola","Chao","Adios"];    
    println!("VALOR array: {:?} ", s);
    println!("VALOR array index: {} ", s[1]);    


} 