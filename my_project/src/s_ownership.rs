struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("page = {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("rating = {:?}", book.rating);
}

pub fn main() {
    let book = Book {
        pages: 333,
        rating: 9,
    };

    display_page_count(&book);
    display_rating(&book);
}
