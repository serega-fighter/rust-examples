
#[derive(Debug)]
struct Holder {
    ctr: String
}

impl Holder {
    pub fn get(self) -> &str {
        return self.ctr.as
    }

    pub fn mutate(&mut self, val: String) {
        self.ctr = val
    }
}

fn main() {
    let mut v = vec![
        Holder{ctr: "Abc".to_string()},
        Holder{ctr: "cdef"},
        Holder{ctr: "zvvx"},
        Holder{ctr: "zvw2"},
        Holder{ctr: "vbs2"}
    ];
    let x: Option<&Holder> = v.get(0);
    println!("X: {:?}", x.unwrap());
    let y = v.get(1);
    println!("Y: {:?}", y.unwrap());
}