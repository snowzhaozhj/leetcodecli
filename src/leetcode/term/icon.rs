pub enum Icon {
    Empty,
    Yes,
    No,
    Star,
    Unstar,
    Lock,
}

impl ToString for Icon {
    fn to_string(&self) -> String {
        match self {
            Icon::Empty => " ".to_string(),
            Icon::Yes => "✔".to_string(),
            Icon::No => "✘".to_string(),
            Icon::Star => "★".to_string(),
            Icon::Unstar => "☆".to_string(),
            Icon::Lock => "🔒".to_string(),
        }
    }
}
