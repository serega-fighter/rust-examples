
struct Holder {
    ctr: u64
}

impl Holder {
    pub fn get(self) -> u64 {
        return self.ctr
    }

    pub fn mutate(&mut self, val: u64) {
        self.ctr = val
    }
}

fn main() {
    let mut v = vec![Holder{ctr: 5}, Holder{ctr: 6}, Holder{ctr: 3}, Holder{ctr: 7}, Holder{ctr: 8}];
    let x: Option<&Holder> = v.get(0);
    println!("X: {}", x.get());
    let y = v.get(1);
    println!("Y: {}", y.get());

}