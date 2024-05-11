mod services;
pub mod utilitys {
    use super::services;

    pub fn get_services() -> String {
        return String::from(services::obtener_servicio());
    }
}
