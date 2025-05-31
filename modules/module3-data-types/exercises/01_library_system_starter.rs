// Define the Book struct
#[derive(Clone)]
struct Book {
    // TODO: Add fields for book properties (title, author, year, isbn)
    
    title: String,
    author: String,
    year: u32,
    isbn: String,

}
impl Book {
     fn new(title: &str, author: &str, year: u32, isbn: &str) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
            isbn: isbn.to_string(),
        }
    }
}


// Define a BookStatus enum to track availability
enum BookStatus {
    // TODO: Add variants for different states (Available, Borrowed)
    Available,
    Borrowed,
}
use std::collections::HashMap;
// Define a Library struct to manage books
struct Library {
    books: HashMap<String, (Book, BookStatus)>, // Keyed by ISBN
    // TODO: Add fields to store books and their status
}

// TODO: Implement methods for the Library struct
impl Library {
    // Create a new, empty library
   fn new() -> Library {
        Library {
            books: HashMap::new(),
        }
    }

    // Add a book to the library
    fn add_book(&mut self, book: Book) {
        // TODO: Implement this function
        self.books.insert(book.isbn.clone(), (book, BookStatus::Available));
    }

    // Borrow a book from the library
    fn borrow_book(&mut self, isbn: &str) -> Result<&Book, &str> {
        // TODO: Implement
        // Check if the book exists in the library
        if isbn.is_empty() {
            return Err("ISBN cannot be empty.");
        }
        else 
            if isbn.len() != 13 {
                return Err("ISBN must be 13 characters long.");
            }
        
        else {
            if !self.books.contains_key(isbn) {
                return Err("Book not found.");
            }
            else {
                let entry = self.books.get_mut(isbn).unwrap();
                if let BookStatus::Borrowed = entry.1 {
                    return Err("Book is already borrowed.");
                }
                else {
                    
                    entry.1 = BookStatus::Borrowed;
                    return Ok(&entry.0);
                }
            }
        }
        }
        
        
    

    // Return a borrowed book to the library
    fn return_book(&mut self, isbn: &str) -> Result<&Book, &str> {
        // TODO: Implement this function
        if isbn.is_empty() {
            return Err("ISBN cannot be empty.");
        }
        else 
            if isbn.len() !=13 {
                return Err("ISBN must be 13 characters long.");
            }
        else {
            if !self.books.contains_key(isbn) {
                return Err("Book not found.");
                
            }
            else {
                let entry = self.books.get_mut(isbn).unwrap();
                if let BookStatus::Available = entry.1 {
                    return Err("Book is not  borrowed.");
                }
                else {
                    
                    entry.1 = BookStatus::Available;
                    return Ok(&entry.0);
                }
    }
}
    }

    // List all books in the library with their status
    fn list_books(&self) {
        // TODO: Implement this function
        for (isbn, (book, status)) in &self.books {
            let status_str = match status {
                BookStatus::Available => "Available",
                BookStatus::Borrowed => "Borrowed",
            };
            println!("ISBN: {}, Title: {}, Author: {}, Year: {}, Status: {}",
                        isbn, book.title, book.author, book.year, status_str);
        }

    }
}

fn main() {
   
    // Create a new library
    let mut library = Library::new();
    
    // Add several books to the library
    library.add_book(Book::new(
        "The Rust Programming Language",
        "Steve Klabnik and Carol Nichols",
        2018,
        "9781718500440"
    ));
    
    library.add_book(Book::new(
        "Design Patterns",
        "Erich Gamma et al.",
        1994,
        "9780201633610"
    ));
    
    library.add_book(Book::new(
        "Clean Code",
        "Robert C. Martin",
        2008,
        "9780132350884"
    ));
    
    // List all books
    library.list_books();
    
    // Borrow a book
    println!("Borrowing \"Clean Code\"...");
    match library.borrow_book("9780132350884") {
        Ok(_) => println!("Book borrowed successfully!"),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // List all books again to see the updated status
    library.list_books();
    
    // Return the book
    println!("Returning \"Clean Code\"...");
    match library.return_book("9780132350884") {
        Ok(_) => println!("Book returned successfully!"),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // List all books one more time
    library.list_books();
}