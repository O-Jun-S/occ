/// It represents constant integer.
pub struct ConstInt(i32);


impl ConstInt {
    pub fn new(n: i32) -> ConstInt {
        ConstInt(n)
    }

    pub fn get(&self) -> i32 {
        self.0
    }
}

#[test]
fn constint_test() {
    let expect = 128;
    let num = ConstInt::new(expect);
    assert_eq!(num.get(), expect);
}
