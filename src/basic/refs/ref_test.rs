use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

pub fn change_str(s: &mut String) {
    s.push_str("AAC");
}

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

fn main() {
    let mut s = String::from("AABBC");

    change_str(&mut s);
    println!("The value of x is: {}", s);

    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);

    showTable(&mut table);
}