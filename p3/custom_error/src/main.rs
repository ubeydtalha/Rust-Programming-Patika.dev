
use core::num;
use std::{fmt, error::Error};

#[derive(Debug)]
enum MyCustomError {
    Io(std::io::Error),
    Parse(num:: ParseIntError),
    Other(String)
}


impl fmt::Display for MyCustomError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyCustomError::Io(err) => write!(f, "IO error: {}", err),
            MyCustomError::Parse(err) => write!(f, "Parse error: {}", err),
            MyCustomError::Other(err) => write!(f, "Other error: {}", err),
        }
    }

}

impl std::error::Error for MyCustomError {}

#[derive(Debug)]
pub struct BookError {
    pub message: String,
}


impl fmt::Display for BookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BookError: {}", self.message)
    }
}

impl  Error for BookError {
    
}

struct Library {
    books : Vec<Book>,
}

struct Book {
    id : u32,
    name : String,
    author : String,
    borrowed : bool,
}

impl Library {
    
    fn new() -> Library {
        Library {
            books : Vec::new(),
        }
    }

    fn add_book(&mut self, book : Book) {
        self.books.push(book);
    }

    fn borrow_book(&mut self, book_id : u32) {
        for book in self.books.iter_mut() {
            if book.id == book_id {
                book.borrowed = true;
            }
        }
    }

    fn is_book_browed(&self, book_id : u32) -> bool {
        for book in self.books.iter() {
            if book.id == book_id {
                return book.borrowed;
            }
        }
        false
    }
}



fn borrow_book(book_id : u32 , library : &mut Library) -> Result<(),BookError> {

    if library.is_book_browed(book_id) {
        Err(BookError{
            message : String::from("Sorry, this book is already borrowed"),
        })
    } else {
        library.borrow_book(book_id);
        Ok(())
    }
}

fn main() {
    
    let mut library = Library::new();

    library.add_book(Book{
        id : 1,
        name : String::from("The Lord of the Rings"),
        author : String::from("J. R. R. Tolkien"),
        borrowed : false,
    });

    library.add_book( Book{
        id : 2,
        name : String::from("The Hobbit"),
        author : String::from("J. R. R. Tolkien"),
        borrowed : false,
    });

    let res = borrow_book(1, &mut library).unwrap();

    println!("Book borrowed successfully {:?}", res);

    let res = borrow_book(1, &mut library).unwrap_err();

    println!("Book borrowed successfully {:?}", res);
}
