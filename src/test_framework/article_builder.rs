use std::fmt::Write;

pub const BOUNDARY: &str = "---------------------------123456789012345678901234567";

#[derive(Default)]
pub struct ArticleBuilder<'a> {
    title_o: Option<String>,
    author_o: Option<String>,
    category_o: Option<String>,
    text_o: Option<String>,
    short_text_o: Option<String>,
    related_articles_o: Option<String>,
    image_description_o: Option<String>,
    is_main_o: Option<bool>,
    is_exclusive_o: Option<bool>,
    image_o: Option<(String, &'a [u8], String)>,
    audio_o: Option<(String, &'a [u8], String)>,
}

impl<'a> ArticleBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title_o = Some(title.into());
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author_o = Some(author.into());
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category_o = Some(category.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text_o = Some(text.into());
        self
    }

    pub fn short_text(mut self, short_text: impl Into<String>) -> Self {
        self.short_text_o = Some(short_text.into());
        self
    }

    pub fn related_articles(mut self, related_articles: impl Into<String>) -> Self {
        self.related_articles_o = Some(related_articles.into());
        self
    }

    pub fn image_description(mut self, image_description: impl Into<String>) -> Self {
        self.image_description_o = Some(image_description.into());
        self
    }

    pub fn is_main(mut self, is_main: bool) -> Self {
        self.is_main_o = Some(is_main);
        self
    }

    pub fn is_exclusive(mut self, is_exclusive: bool) -> Self {
        self.is_exclusive_o = Some(is_exclusive);
        self
    }

    pub fn image(
        mut self,
        filename: impl Into<String>,
        data: &'a [u8],
        content_type: &str,
    ) -> Self {
        self.image_o = Some((filename.into(), data, content_type.into()));
        self
    }
    pub fn audio(
        mut self,
        filename: impl Into<String>,
        data: &'a [u8],
        content_type: &str,
    ) -> Self {
        self.audio_o = Some((filename.into(), data, content_type.into()));
        self
    }

    pub fn build(self) -> Result<String, std::fmt::Error> {
        let mut body = String::new();

        if let Some(title) = self.title_o {
            write!(body, "{}", line("title", title))?;
        }

        if let Some(author) = self.author_o {
            write!(body, "{}", line("author", author))?;
        }

        if let Some(category) = self.category_o {
            write!(body, "{}", line("category", category))?;
        }

        if let Some(text) = self.text_o {
            write!(body, "{}", line("text", text))?;
        }

        if let Some(short_text) = self.short_text_o {
            write!(body, "{}", line("short_text", short_text))?;
        }

        if let Some(related_articles) = self.related_articles_o {
            write!(body, "{}", line("related_articles", related_articles))?;
        }

        if let Some(image_description) = self.image_description_o {
            write!(body, "{}", line("image_description", image_description))?;
        }

        if let Some(is_main) = self.is_main_o {
            write!(body, "{}", line("is_main", is_main))?;
        }

        if let Some(is_exclusive) = self.is_exclusive_o {
            write!(body, "{}", line("is_exclusive", is_exclusive))?;
        }

        if let Some((filename, data, content_type)) = self.image_o {
            write!(body, "{}", line_file("image", &filename, &content_type))?;

            body.push_str(&String::from_utf8_lossy(&data));
            writeln!(body, "\r\n")?;
        }

        if let Some((filename, data, content_type)) = self.audio_o {
            write!(body, "{}", line_file("audio", &filename, &content_type))?;

            body.push_str(&String::from_utf8_lossy(&data));
            writeln!(body, "\r\n")?;
        }

        write!(body, "--{}--\r\n", BOUNDARY)?;
        Ok(body)
    }
}

fn line(name: &str, value: impl ToString) -> String {
    format!(
        "--{}\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n",
        BOUNDARY,
        name,
        value.to_string()
    )
}

fn line_file(name: &str, filename: &str, content_type: &str) -> String {
    format!(
        "--{}\r\nContent-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\nContent-Type: {}\r\n\r\n",
        BOUNDARY, name, filename, content_type
    )
}
