fn main() {
    struct Book {
        title: String,
        author: String,
    }

    impl Book {
        fn describe(&self) {
            println!("Title: {}, Author: {}", self.title, self.author);
        }
    }

    let book = Book {
        title: String::from("Konstytucja Związku Radzieckiego"),
        author: String::from("Jan Bogdan"),
    };

    book.describe();
}
