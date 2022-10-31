use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs, io, io::BufRead};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Class {
    Need,
    Want,
    Save,
    Loan,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Classifier {
    classes: HashMap<String, Class>,
    class_file: String,
}

impl Classifier {
    pub fn new(class_file: &str) -> Self {
        Self {
            classes: fs::File::open(class_file)
                .map(|f| serde_json::from_reader(io::BufReader::new(f)).unwrap_or_default())
                .unwrap_or_default(),
            class_file: class_file.to_string(),
        }
    }

    pub fn classify(&mut self, category: &str) -> Class {
        if self.classes.contains_key(category) {
            return self.classes.get(category).unwrap().clone();
        }
        let cls = prompt_class(category);
        self.classes.insert(category.to_string(), cls);
        cls
    }
}

fn prompt_class(category: &str) -> Class {
    loop {
        println!("Category: {category}");
        println!("(1) Want");
        println!("(2) Need");
        println!("(3) Save");
        println!("(4) Loan");
        let mut line = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut line).unwrap();
        if let Ok(choice) = line.trim().parse::<i32>() {
            if choice == 1 {
                return Class::Want;
            }
            if choice == 2 {
                return Class::Need;
            }
            if choice == 3 {
                return Class::Save;
            }
            if choice == 4 {
                return Class::Loan;
            }
        }
    }
}

impl Drop for Classifier {
    fn drop(&mut self) {
        fs::File::create(&self.class_file)
            .map(|f| {
                serde_json::to_writer(io::BufWriter::new(f), &self.classes)
                    .expect("Failed to write category classifications");
            })
            .expect("Failed to create category classifications");
    }
}
