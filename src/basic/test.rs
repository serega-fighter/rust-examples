
pub struct TestStruct {

}

impl TestStruct {
    pub fn test_func_2() {
        println!("Test func 2");
    }
}

pub fn test_func() {
    println!("That's a test func");
}

fn main() {
    println!("The value of x is: {}", 55);
    TestStruct::test_func_2();
}