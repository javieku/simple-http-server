use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    parameters: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get_next_word(&self, key: &str) -> Option<&Value> {
        self.parameters.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(idx) = sub_str.find("=") {
                key = &sub_str[..idx];
                val = &sub_str[idx + 1..];
            }
            data.entry(key)
                .and_modify(|existing| match existing {
                    Value::Single(prev_value) => {
                        let v = vec![prev_value, val];
                        *existing = Value::Multiple(v);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }
        QueryString { parameters: data }
    }
}
