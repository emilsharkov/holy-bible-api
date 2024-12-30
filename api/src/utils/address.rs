use crate::config::settings;

pub fn get_app_address(settings: &settings::Settings) -> String {
    let settings::AppSettings { host, port } = &settings.app_setting;
    let addr = format!("{}:{}", host, port);
    return addr;
}