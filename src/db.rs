use std::collections::HashMap;

pub struct DataBase {
    inner: HashMap<String, Vec<u8>>,
}

impl DataBase {
    pub fn new() -> Self {
        let inner: HashMap<_, _> = [
            (String::from("codecrafters.io"), vec![12, 34, 56, 78]),
            (
                String::from("abc.longassdomainname.com"),
                vec![90, 112, 123, 134],
            ),
            (
                String::from("def.longassdomainname.com"),
                vec![112, 123, 134, 145],
            ),
        ]
        .into_iter()
        .collect();
        Self { inner }
    }

    pub fn get(&self, domain_name: &str) -> Option<Vec<u8>> {
        self.inner.get(domain_name).cloned()
    }
}
