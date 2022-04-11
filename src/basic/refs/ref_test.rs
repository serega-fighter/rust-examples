use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

pub fn showTable(table: &mut Table) {
    for (artist, works) in table {

        let ar: &String = artist;
        println!("works by {}:", ar);

        let w: &Vec<String> = works; // casted to ref

        for work in w {
            println!(" {}", work);
        }
    }
}

pub fn showVec(v: &Vec<String>) {
    for w in v {
        println!("val: {}", w);
    }
}

fn factorial(n: usize) -> usize {
    (1..n+1).fold(1, |a, b| a * b)
}

fn main() {

    // Simple case
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);

    showTable(&mut table);

    // Borrow and reassign to modify vector
    let mut vec = Vec::new();
    vec.push("ABC".to_string());
    vec.push("BCA".to_string());
    vec.push("CDE".to_string());
    vec.push("DEF".to_string());

    let s = &mut vec[2];
    *s = "ZXV".to_string();

    showVec(&vec);

    // Reference to expression
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);
}