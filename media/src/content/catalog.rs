use crate::content::media::Media;

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    pub(crate) fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    pub(crate) fn add(&mut self, item: Media) {
        self.items.push(item);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        match self.items.get(index) {
            Some(item) => Some(item),
            None => None,
        }
    }

    pub fn display_by_index(&self, index: usize) {
        match self.items.get(index) {
            Some(item) => println!("{}", item.description()),
            None => println!("Item not found"),
        }
    }

    pub fn display(&self) {
        for item in &self.items {
            println!("{}", item.description());
        }
    }
}
