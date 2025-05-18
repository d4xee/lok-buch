use std::fmt::Display;

#[derive(Default, PartialEq, Debug, Clone, Eq, Hash)]
pub enum Languages {
    #[default]
    English,
    German,
}

impl Languages {
    pub(crate) const ALL: [Languages; 2] = [Languages::English, Languages::German];

    pub fn short_language_code(&self) -> String {
        match self {
            Languages::English => "en".to_owned(),
            Languages::German => "de".to_owned(),
        }
    }
}

impl Display for Languages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let language = t!(format!("lang.{}", self.short_language_code())).into_owned();
        write!(f, "{}", language)
    }
}
