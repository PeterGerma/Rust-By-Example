//All types can derive the fmt::Debug implementation.
#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    //This implementation require fmt with this exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    //In general, the {} will autmatically be replaced by stringified arguments
    println!("{} days", 31);

    //Positional Arguments
    println!("{0}, this is {1}.  {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over");

    println!("Now  {:?} will print!", Structure(3));


}