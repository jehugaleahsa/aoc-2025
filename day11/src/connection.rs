#[derive(Debug)]
pub struct Connection {
    pub input: String,
    pub output: String,
}

impl Connection {
    #[must_use]
    pub fn parse(value: &str) -> Option<Vec<Self>> {
        let (input, outputs) = value.split_once(':')?;
        if input.is_empty() {
            return None;
        }
        let outputs = outputs.trim_ascii();
        let outputs = outputs.split(' ').collect::<Vec<&str>>();
        let mut connections = Vec::new();
        for output in outputs {
            let output = output.trim_ascii();
            if output.is_empty() {
                return None;
            }
            let connection = Connection {
                input: input.to_string(),
                output: output.to_string(),
            };
            connections.push(connection);
        }
        Some(connections)
    }
}
