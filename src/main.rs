#[derive(Debug)]
pub struct  Book {
    name: String,
    isbn: i32
}

#[derive(Debug)]
pub struct  BookDaoImpl {
    books: Vec<Book>
}

impl Book {
    pub fn new(isbn: i32, name: String) -> Self {
        Self {
            isbn,
            name
        }
    }
}

impl BookDaoImpl {
    pub fn new() -> Self {
        let mut vb = Vec::new();

        vb.push(Book::new(1, "murilo".to_string()));
        vb.push(Book::new(2, "da".to_string()));
        vb.push(Book::new(3, "ijanc".to_string()));

        Self {
            books: vb
        }
    }
}

trait BookDao {
    fn get_all_books(&mut self) -> &Vec<Book>;
    fn save_book(&mut self, book: Book);
}

impl BookDao for BookDaoImpl {
    // get all books
    fn get_all_books(&mut self) -> &Vec<Book> {
        &self.books
    }

    fn save_book(&mut self, book: Book) {
        self.books.push(book);
    }
}

fn main() {
    let b: Book = Book::new(4, "ana luiza".to_string());
    let mut book_dao = BookDaoImpl::new();

    book_dao.save_book(b);

    for bo in book_dao.get_all_books() {
        println!("Book: {:?}", bo);
    }
}
