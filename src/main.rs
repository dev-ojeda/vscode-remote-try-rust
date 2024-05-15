#![allow(unused)]

use task::Task;
use utilitys::prelude::*;
use utilitys::web::{Request, RequestBuilder};
pub mod task;
pub mod utilitys;

pub fn main() -> Result<()> {
    //let salida = utilitys::utilitys::handle_error();
    // let task = Task::new("Task 01");
    //let task = Task::default();
    let req = RequestBuilder::empty()
        .url("https://some-url.com/task/123")
        .method("GET")
        .headers("token", "user_uuid.exp.sign")
        .build()?;
    println!("{req:#?}");
    Ok(())
    // let url = "/create/user";
    // let metodo = "POST";
    // let respuesta: (u16, &str) = {
    //     let mut param = services::ParametrosServices::new(url, metodo);

    //     let res = services::AccionesServices::handle_request(&mut param);

    //     return res;
    // };
    // println!("MSG: {:?}", respuesta);
}
