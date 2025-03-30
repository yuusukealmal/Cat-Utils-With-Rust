use std::time::Duration;

pub fn count_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs_f64();

    if total_secs < 60.0 {
        format!("{:.2}秒", total_secs)
    } else if total_secs < 3600.0 {
        let total_secs = duration.as_secs();
        let minutes = total_secs / 60;
        let seconds = total_secs % 60;
        format!("{}:{:02}分鐘", minutes, seconds)
    } else {
        let total_secs = duration.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        format!("{:02}:{:02}:{:02}小時", hours, minutes, seconds)
    }
}
