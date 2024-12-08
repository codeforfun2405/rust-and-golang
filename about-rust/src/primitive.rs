#[derive(Debug)]
pub enum Status {
    Pending,
    Schedued,
    Running,
    Stopped,
}

pub fn data() {
    let num1: i8 = 1;
    let num2: i16 = 1;
    let num3: i32 = 1;
    let num4: i64 = 1;
    let num5: i128 = 1;
    println!("{}, {}, {}, {}, {}", num1, num2, num3, num4, num5);

    let num1: u8 = 1;
    let num2: u16 = 1;
    let num3: u32 = 1;
    let num4: u64 = 1;
    let num5: u128 = 1;
    println!("{}, {}, {}, {}, {}", num1, num2, num3, num4, num5);
}

pub fn strings() {
    let s1 = "Good Evening";
    let s2 = String::from("Hello Nice");
    let ch = 'H';
    println!("{}, {}, {}", s1, s2, ch);
}
