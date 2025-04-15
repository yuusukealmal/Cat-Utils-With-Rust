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

    #[derive(PartialEq, Clone, Debug)]
    #[allow(dead_code)]
    pub enum RareLevel {
        Normal,
        EX,
        Rare,
        SuperRare,
        UberRare,
        Legend,
    }
    pub fn log_gatya(level: &RareLevel, message: String) {
        match level {
            RareLevel::Normal => println!("{}", message),
            RareLevel::EX => println!("{}", message.yellow()),
            RareLevel::Rare => println!("{}", message.green()),
            RareLevel::SuperRare => println!("{}", message.blue()),
            RareLevel::UberRare => println!("{}", message.magenta()),
            RareLevel::Legend => println!("{}", message.red()),
        }
    }
}
