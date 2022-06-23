use ernd::TinyString;

fn main() {
    let mut string = TinyString::new();

    string.push_str("massively ");
    string.push_str("fat ");
    string.push_str("sex");

    println!("{string:?}");
}
