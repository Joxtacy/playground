use std::{collections::HashMap, fmt::Display, fs, hash::Hash};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut conn = Connection::<String, String>::open("./my.db");
        println!("DEBUG: {:?}", conn.data);

        conn.add("Key".to_string(), "Value".to_string());
        conn.add("another".to_string(), "value".to_string());
        assert_eq!(conn.path, "./my.db".to_string());
        println!("End of test");
    }
}

pub struct Connection<'a, K, V>
where
    K: Display + Hash + std::cmp::Eq + serde::Serialize + serde::Deserialize<'a>,
    V: Display + serde::Serialize,
{
    path: String,
    data: HashMap<K, V>,
}

impl<'a, K, V> Connection<'a, K, V>
where
    K: Display + Eq + Hash + From<std::string::String> + serde::Serialize + serde::Deserialize<'a>,
    V: Display + From<std::string::String> + serde::Serialize,
{
    pub fn open(path: &str) -> Connection<'a, K, V> {
        let contents = match fs::read_to_string(path) {
            Ok(contents) => contents.as_bytes(),
            Err(e) => {
                eprintln!("Could not open file. Reason: {:?}", e);
                String::from("").as_bytes()
            }
        };
        let deserialized: HashMap<K, V> = bincode::deserialize(&contents).expect("rip");
        let mut data = HashMap::new();
        contents
            .lines()
            .map(|line| line.split(':').map(String::from).collect())
            .for_each(|s: Vec<String>| {
                data.insert(
                    s.first().unwrap().to_owned().into(),
                    s.last().unwrap().to_owned().into(),
                );
            });
        Connection {
            path: path.to_string(),
            data,
        }
    }

    pub fn add(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }
}

impl<'a, K, V> Drop for Connection<'a, K, V>
where
    K: Display + Hash + std::cmp::Eq + serde::Serialize + serde::Deserialize<'a>,
    V: Display + serde::Serialize,
{
    fn drop(&mut self) {
        let contents = bincode::serialize(&self.data);
        if let Ok(contents) = contents {
            if let Err(e) = fs::write(&self.path, contents) {
                eprintln!("Could not write to file. Reason: {:?}", e);
            }
        } else {
            eprintln!("No contents to write!");
        }
    }
}
