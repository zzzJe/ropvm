pub(super) fn get_current_time() -> String {
    let date = chrono::Local::now();
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}
