#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("Book: {} by {}", title, author),
            Media::Movie { title, director } => format!("Movie: {} by {}", title, director),
            Media::Audiobook { title } => format!("Audiobook: {}", title),
            Media::Podcast(episode) => format!("Podcast: Episode {}", episode),
            Media::Placeholder => "Placeholder".to_string(),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    fn add(&mut self, item: Media) {
        self.items.push(item);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        match self.items.get(index) {
            Some(item) => Some(item),
            None => None,
        }
    }

    fn display_by_index(&self, index: usize) {
        match self.items.get(index) {
            Some(item) => println!("{}", item.description()),
            None => println!("Item not found"),
        }
    }

    fn display(&self) {
        for item in &self.items {
            println!("{}", item.description());
        }
    }
}

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
