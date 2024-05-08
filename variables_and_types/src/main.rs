fn main() {
    println!("Hello, world!");
    integers()
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
