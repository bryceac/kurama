pub struct Post {
    title: String,
    date: NaiveDate,
    content: String,
    slug: String
}

impl Post {
    pub fn from_page(p: Page) -> Self {
        Self {
            title: p.metadata.title,
            date: p.metadata.date.unwrap(),
            content: p.content,
            slug: p.metadata.slug
        }
    }
}