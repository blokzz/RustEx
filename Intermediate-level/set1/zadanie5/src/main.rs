enum LogLevel {
    Info,
    Warning,
    Error,
}

struct LogMessage {
    level: LogLevel,
    message: String,
}

fn log(log_message: &LogMessage) -> String {
    match log_message.level {
        LogLevel::Info => format!("[INFO]: {}", log_message.message),
        LogLevel::Warning => format!("[WARNING]: {}", log_message.message),
        LogLevel::Error => format!("[ERROR]: {}", log_message.message),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_info_log() {
        let message = LogMessage {
            level: LogLevel::Info,
            message: String::from("Test message"),
        };
        assert_eq!(log(&message), "[INFO]: Test message");
    }

    #[test]
    fn test_warning_log() {
        let message = LogMessage {
            level: LogLevel::Warning,
            message: String::from("Test warning"),
        };
        assert_eq!(log(&message), "[WARNING]: Test warning");
    }

    #[test]
    fn test_error_log() {
        let message = LogMessage {
            level: LogLevel::Error,
            message: String::from("Test error"),
        };
        assert_eq!(log(&message), "[ERROR]: Test error");
    }
}