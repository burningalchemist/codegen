use std::fmt::{self, Write};

use fields::Fields;
use formatter::Formatter;

use r#type::Type;

/// Defines an enum variant.
#[derive(Debug, Clone)]
pub struct Variant {
    name: String,
    fields: Fields,
    documentation: Vec<String>,
}

impl Variant {
    /// Return a new enum variant with the given name.
    pub fn new(name: &str) -> Self {
        Variant {
            name: name.to_string(),
            fields: Fields::Empty,
            documentation: Vec::new(),
        }
    }

    /// Add a named field to the variant.
    pub fn named<T>(&mut self, name: &str, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.fields.named(name, ty);
        self
    }

    /// Add a tuple field to the variant.
    pub fn tuple(&mut self, ty: &str) -> &mut Self {
        self.fields.tuple(ty);
        self
    }
    /// Add a documentation to the variant
    pub fn doc(&mut self, documentation: Vec<&str>) -> &mut Self {
        self.documentation = documentation.iter().map(|doc| doc.to_string()).collect();
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if !self.documentation.is_empty() {
            for doc in &self.documentation {
                write!(fmt, "/// {}\n", doc)?;
            }
        }
        write!(fmt, "{}", self.name)?;
        self.fields.fmt(fmt)?;
        write!(fmt, ",\n")?;

        Ok(())
    }
}
