mod content;

use content::media::Media;
use content::catalog::Catalog;

fn main() {
    let audiobook = Media::Audiobook {
        title: "The Great Gatsby".to_string(),
    };

    let book = Media::Book {
        title: "The Lord of the Rings".to_string(),
        author: "J.R.R. Tolkien".to_string(),
    };

    let movie = Media::Movie {
        title: "A Nightmare on Elm Street".to_string(),
        director: "Wes Craven".to_string(),
    };

    let podcast = Media::Podcast(1);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(book);
    catalog.add(movie);
    catalog.add(podcast);
    catalog.add(placeholder);

    // let item = catalog.get_by_index(100);
    // println!("{:?}", item.unwrap_or(&Media::Placeholder));
    // println!("{:?}", item.expect("Item not found"));

    catalog.display_by_index(0);
    catalog.display_by_index(1);
    catalog.display_by_index(2);
    catalog.display_by_index(3);
    catalog.display_by_index(4);
    catalog.display_by_index(100);

    // catalog.display();
}
