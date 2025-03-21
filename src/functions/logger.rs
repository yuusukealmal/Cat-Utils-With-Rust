pub mod logger {
    use colored::Colorize;

    #[allow(dead_code)]
    pub enum LogLevel {
        Info,
        Warning,
        Error,
        Debug,
    }

    pub fn log(level: LogLevel, message: String) {
        match level {
            LogLevel::Info => println!("{} {}", "[Info]".green(), message),
            LogLevel::Warning => println!("{} {}", "[Warning]".yellow(), message),
            LogLevel::Error => println!("{} {}", "[Error]".red(), message),
            LogLevel::Debug => println!("{} {}", "[Debug]".blue(), message),
        }
    }
}
