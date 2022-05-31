
pub fn fib(mut n: i64) -> f64 {
    let mut a: i64 = 1;
    let mut b: i64 = 0;
    let mut _temp: i64 = 0;

    while n >= 0 {
        _temp = a;
        a = a + b;
        b = _temp;
        n -= 1;
    }
    b as f64
}
 