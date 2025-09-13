pub fn log_error(err: &anyhow::Error) {
    log::error!("Error: {:?}", err);
}

pub fn log_info(msg: &str) {
    log::info!("{}", msg);
}
