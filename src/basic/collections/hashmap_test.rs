use std::collections::HashMap;

/*
By default, HashMap uses a hashing algorithm selected to provide resistance against HashDoS attacks.
The algorithm is randomly seeded, and a reasonable best-effort is made to generate this seed
from a high quality, secure source of randomness provided by the host without blocking the program.
Because of this, the randomness of the seed depends on the output quality of the system’s random
number generator when the seed is created. In particular, seeds generated when the system’s entropy
pool is abnormally low such as during system boot may be of a lower quality.

The default hashing algorithm is currently SipHash 1-3, though this is subject to change at any
point in the future. While its performance is very competitive for medium sized keys, other
hashing algorithms will outperform it for small keys such as integers as well as large
keys such as long strings, though those algorithms will typically not protect against attacks
such as HashDoS.

The hash table implementation is a Rust port of Google’s SwissTable.
*/

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking { name: name.to_string(), country: country.to_string() }
    }
}

fn main() {

    let mut vikings = HashMap::new();
    vikings.insert(Viking::new("Einar", "Norway"), 5);
    vikings.insert(Viking::new("Olaf", "Denmark"), 10);

    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }

    let k = Viking::new("Einar", "Norway");
    println!("Contains key: {}", vikings.contains_key(&k));

    let queried = vikings.get(&k);
    match queried {
        Some(x) => println!("Value found {}", x),
        None => println!("Value not found")
    }

    // Will panic if the key is not found
    println!("Query key: {}", vikings[&k]);

    vikings.remove(&k);
    println!("Contains key: {}", vikings.contains_key(&k));

}