#[derive(Default, Debug, Clone, Copy)]
pub struct CmdLine<'a> {
    data: &'a str,
}

impl<'a> CmdLine<'a> {
    pub const fn new(data: &'a str) -> Self {
        return Self { data };
    }

    pub fn inner(&self) -> &'a str {
        self.data
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'a str, Option<&'a str>)> {
        let mut idx = 0;

        core::iter::from_fn(move || {
            // Skip leading whitespace
            idx = self.data[idx..]
                .find(|c: char| !c.is_whitespace())
                .map(|i| idx + i)
                .unwrap_or(self.data.len());

            // If we're at the end of the command line string, return None
            if idx >= self.data.len() {
                return None;
            }

            // Save the current index and find the next equal sign or whitespace
            let start = idx;
            let end = self.data[start..]
                .find(|c: char| c == '=' || c.is_whitespace())
                .map(|i| start + i)
                .unwrap_or(self.data.len());

            idx = end;

            let name = &self.data[start..end];

            // If the remaining string is empty or doesn't start with an equal sign,
            // return the substring from start to end. That means we found a name
            // without a value (boolean)
            if self.data[idx..].is_empty() || !self.data[idx..].starts_with('=') {
                Some((name, None))
            } else {
                // Skip the equal sign
                idx += 1;

                // Check if the value is a quoted string
                let quote = if self.data[idx..].starts_with('"') {
                    Some('"')
                } else if self.data[idx..].starts_with('\'') {
                    Some('\'')
                } else {
                    None
                };

                if quote.is_some() {
                    // Skip the opening quote
                    idx += 1;
                }

                // Find the end of the value
                let start = idx;
                let end = match quote {
                    Some(quote) => {
                        // Find the closing quote
                        self.data[start..]
                            .find(quote)
                            .map(|i| start + i)
                            .unwrap_or(self.data.len())
                    }
                    None => {
                        // Find the next whitespace or end of string
                        self.data[start..]
                            .find(|c: char| c.is_whitespace())
                            .map(|i| start + i)
                            .unwrap_or(self.data.len())
                    }
                };

                idx = end;

                // If we found a closing quote, skip it
                if quote.is_some() && end < self.data.len() {
                    idx += 1;
                }

                // Return the parsed key and value
                let value = &self.data[start..end];

                Some((name, Some(value)))
            }
        })
    }

    pub fn get_string(&self, name: &str) -> Option<&'a str> {
        self.iter()
            .find(|(key, _)| *key == name)
            .and_then(|(_, value)| value)
    }

    pub fn get_bool(&self, name: &str) -> Option<bool> {
        match self.get_string(name)? {
            "true" | "yes" | "on" | "1" => Some(true),
            "false" | "no" | "off" | "0" => Some(false),
            value => {
                warn!("Invalid boolean value for {}: {:?}", name, value);
                None
            }
        }
    }

    pub fn get_usize(&self, name: &str) -> Option<usize> {
        let value = self.get_string(name)?;

        match value.parse::<usize>() {
            Ok(value) => Some(value),
            Err(_) => {
                warn!("Invalid usize value for {}: {:?}", name, value);
                None
            }
        }
    }
}
