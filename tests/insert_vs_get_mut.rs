use std::collections::HashMap;

pub struct Record {
    name: String,
    grade: char,
    age: i8,
}

pub fn generate_my_records() -> HashMap<String, Record> {
    let mut my_records = HashMap::new();
    my_records.insert(
        "first".to_string(),
        Record {
            name: "Fred".to_string(),
            grade: 'B',
            age: 20,
        },
    );
    my_records.insert(
        "second".to_string(),
        Record {
            name: "Bob".to_string(),
            grade: 'C',
            age: 20,
        },
    );
    my_records.insert(
        "third".to_string(),
        Record {
            name: "Clara".to_string(),
            grade: 'A',
            age: 20,
        },
    );
    my_records.insert(
        "forth".to_string(),
        Record {
            name: "Jane".to_string(),
            grade: 'B',
            age: 20,
        },
    );
    my_records
}


pub fn check_map_insert(map: &mut HashMap<String, Record>) {
    let second = map.entry("second".to_string()).or_insert(Record {
        name: String::new(),
        grade: 'X',
        age: 0,
    });
    second.grade = 'A';
    second.age = 21;
}

pub fn check_map_get_mut(map: HashMap<String, Record>) {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        let mut my_records = generate_my_records();
        let my_records_expected = check_map_insert(&mut my_records);
        let second = my_records.get("second").unwrap();
        assert_eq!(second.grade, 'A');
        assert_eq!(second.age, 21);
    }
}
