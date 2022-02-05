use std::collections::HashMap;
use std::string::ToString;

pub struct Command {
    hostname: String,
    port: u16,
    command_name: String,
    params: HashMap<String, String>,
}

impl Command {
    /// Create a new BluOS device from hostname + port
    pub fn new(hostname: &str, port: u16, command_name: &str) -> Command {
        Command {
            hostname: hostname.to_string(),
            port,
            command_name: command_name.to_string(),
            params: HashMap::new(),
        }
    }

    pub fn add_param<T: ToString>(&mut self, param: &str, value: T) {
        self.params.insert(param.to_string(), value.to_string());
    }

    pub fn add_optional<T: ToString>(&mut self, param: &str, value: Option<T>) {
        match value {
            Some(v) => self.add_param(param, v),
            None => {}
        }
    }

    pub fn build(&self) -> String {
        let base = format!(
            "http://{}:{}/{}",
            self.hostname, self.port, self.command_name
        );
        let mut query = String::new();
        if self.params.len() > 0 {
            query.push('?');
            for (key, value) in self.params.iter() {
                query.push_str(format!("{}={}&", key, value).as_str());
            }
        }
        format!("{}{}", base, query)
    }
}

#[cfg(test)]
mod tests {
    use super::Command;

    #[test]
    fn simple_command() {
        let cmd = Command::new("korv", 1515, "Hello");
        let result = cmd.build();
        assert_eq!(result, "http://korv:1515/Hello")
    }

    #[test]
    fn command_build() {
        let mut cmd = Command::new("korv", 10000, "Hello");
        cmd.add_param("int", 123);
        cmd.add_param("string", "lol");
        let result = cmd.build();
        assert!(result.contains("int=123"));
        assert!(result.contains("string=lol"));
    }
}
