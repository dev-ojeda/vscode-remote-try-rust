pub mod utilitys;

pub fn main() {
    let mensaje = utilitys::utilitys::get_services();
    println!("MSG: {}", mensaje);
}
