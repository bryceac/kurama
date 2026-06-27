use serde::{ Serialize, Deserialize};

use crate::paginator::Paginator;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Archive {
    pub page: usize
}

impl Archive {
    pub fn render(&self, config: &Configuration, templates: &LazyLock<Tera>, paginator: &Paginator) -> Result<String, String> {
        let mut context = Context::new();
        context.insert("site", &config);
        context.insert("page", &self);
        context.insert("posts", &paginator.page(self.page));
        context.insert("pages", &paginator.page_count());
    
        if let Some(sections) = menu_from::<Section>("links.json") {
            context.insert("sections", &sections);
        } else if let Some(links) = menu_from::<Link>("links.json") {
            context.insert("links", &links);
        }

        match templates.render("archive.html", &context) {
            Ok(output) => Ok(format!("{:#}", output)),
            Err(errors) => Err(format!("{}", errors))
        }
    }
}