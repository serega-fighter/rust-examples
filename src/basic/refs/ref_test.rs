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

fn main() {

    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);

    showTable(&mut table);

    let mut vec = Vec::new();
    vec.push("ABC".to_string());
    vec.push("BCA".to_string());
    vec.push("CDE".to_string());
    vec.push("DEF".to_string());

    let s = &mut vec[2];
    *s = "ZXV".to_string();

    showVec(&vec);
}