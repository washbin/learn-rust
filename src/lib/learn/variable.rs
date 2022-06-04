pub fn variables() {
    let x = 5;
    println!("The value of x is: {}, {}, {}", x, &x, &&x);
    let byte1 = b'A';
    let byte2 = b'B';
    let char = 'b';
    let _tup = (300, 6.4, 'A');
    println!("{} {} {} {:0b}", byte1, char, byte2, byte2);
}
