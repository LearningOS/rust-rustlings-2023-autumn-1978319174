// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.
struct Book<'a, 'b> {
    author: &'a str,
    title: &'b str,
}

fn main() {
    let name = String::from("Jill Smith");
    let m;
    {
        let title = String::from("Fish Flying");
        let book = Book { author: &name, title: &title };
        m = book.author;
    }

    println!("{}", m);
    // println!("{} by {}", name, title);
}
