use gbc::Gbc;

fn main() {
    let e = 0b1010_1010;
    let r: u8 = e << 1;

    println!("{e:#010b}\n{r:#010b}");
}
