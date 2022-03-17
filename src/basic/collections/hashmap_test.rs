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

fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    for (key, value) in &scores {
        println!("{} -> {}", key, value);
    }
}