fn main() {
    println!("Hello, world!");
    integers();
    floats();
    other_simple_types();
    //some_techniques()
}
fn integers() {
    //signed types
    let a1: i32 = -12345;
    let a2: i32 = 0xffff;
    let a3: i64 = 0o117;
    let a4: i32 = 0b1110;

    //unsigned types
    let b1: u32 = 12345;
    let b2: u64 = 0xfdfdfd;

    //platform specific integer types isize and usize
    let c1: isize = -1256;
    let c2: usize = 0x1f4d;
    println!(
        "\n The Numbers are :: Signed::{}, {}, {}, {}, unsigned:: {}, {}, platform::{}, {}",
        a1, a2, a3, a4, b1, b2, c1, c2
    );
    println!(
        "\nThe Numbers in reverse order are: {7}, {6}, {5}, {4}, {3}, {2}, {1}, {0}",
        a1, a2, a3, a4, b1, b2, c1, c2
    );
    println!(
        "\nWord size is {} bytes in my machine",
        std::mem::size_of::<isize>()
    )
}
fn floats() {
    let f1: f32 = 1.23456;
    let f2 = 9.87654;

    println!("\nFloats are {} and {}", f1, f2);
    println!("\nFloats in 2dp are {:.2} and {:.2}", f1, f2);
    println!("\nFloats with field width 10 L-aligned, filled with space are ***{:<10}*** and ***{:<10}***", f1, f2);
    println!("\nFloats with field width 10, R-aligned and filled with $ are ***{:$>10}*** and ***{:$>10}***", f1, f2);
}
fn other_simple_types() {
    let is_jithin: bool = true;
    let can_sing = false; //Type inference from value

    let fav_char: char = 'J';
    let fav_emoji = '┼';

    println!("\nIs Jihin? {}, can he sing ? {}, His fav char is {} and fav emoji is {}\n Size of char in rus is {} bytes\n", is_jithin, can_sing, fav_char, fav_emoji, std::mem::size_of::<char>());
}
