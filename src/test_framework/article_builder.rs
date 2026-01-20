use std::io::Write;

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

    pub fn title(mut self, title: &str) -> Self {
        self.title_o = Some(title.into());
        self
    }

    pub fn author(mut self, author: &str) -> Self {
        self.author_o = Some(author.into());
        self
    }

    pub fn category(mut self, category: &str) -> Self {
        self.category_o = Some(category.into());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text_o = Some(text.into());
        self
    }

    pub fn short_text(mut self, short_text: &str) -> Self {
        self.short_text_o = Some(short_text.into());
        self
    }

    pub fn related_articles(mut self, related_articles: &str) -> Self {
        self.related_articles_o = Some(related_articles.into());
        self
    }

    pub fn image_description(mut self, image_description: &str) -> Self {
        self.image_description_o = Some(image_description.into());
        self
    }

    pub fn main(mut self) -> Self {
        self.is_main_o = Some(true);
        self
    }

    pub fn exclusive(mut self) -> Self {
        self.is_exclusive_o = Some(true);
        self
    }

    pub fn image(
        mut self,
        filename: &str,
        data: &'a [u8],
        content_type: &str,
    ) -> Self {
        self.image_o = Some((filename.into(), data, content_type.into()));
        self
    }
    pub fn audio(
        mut self,
        filename: &str,
        data: &'a [u8],
        content_type: &str,
    ) -> Self {
        self.audio_o = Some((filename.into(), data, content_type.into()));
        self
    }

    pub fn build(self) -> std::io::Result<Vec<u8>> {
        let mut body: Vec<u8> = Vec::new();

        macro_rules! text_part {
            ($name:expr, $val:expr) => {
                write!(
                    body,
                    "--{}\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n",
                    BOUNDARY, $name, $val
                )?;
            };
        }

        if let Some(v) = self.title_o {
            text_part!("title", v);
        }
        if let Some(v) = self.author_o {
            text_part!("author", v);
        }
        if let Some(v) = self.category_o {
            text_part!("category", v);
        }
        if let Some(v) = self.text_o {
            text_part!("text", v);
        }
        if let Some(v) = self.short_text_o {
            text_part!("short_text", v);
        }
        if let Some(v) = self.related_articles_o {
            text_part!("related_articles", v);
        }
        if let Some(v) = self.image_description_o {
            text_part!("image_description", v);
        }
        if let Some(v) = self.is_main_o {
            text_part!("is_main", if v { "on" } else { "off" });
        }
        if let Some(v) = self.is_exclusive_o {
            text_part!("is_exclusive", if v { "on" } else { "off" });
        }

        // image
        if let Some((filename, data, content_type)) = self.image_o {
            write!(
                body,
                "--{}\r\nContent-Disposition: form-data; name=\"image\"; filename=\"{}\"\r\nContent-Type: {}\r\n\r\n",
                BOUNDARY, filename, content_type
            )?;
            body.extend_from_slice(data);
            body.extend_from_slice(b"\r\n");
        }

        // audio
        if let Some((filename, data, content_type)) = self.audio_o {
            write!(
                body,
                "--{}\r\nContent-Disposition: form-data; name=\"audio\"; filename=\"{}\"\r\nContent-Type: {}\r\n\r\n",
                BOUNDARY, filename, content_type
            )?;
            body.extend_from_slice(data);
            body.extend_from_slice(b"\r\n");
        }

        write!(body, "--{}--\r\n", BOUNDARY)?;
        Ok(body)
    }
}
