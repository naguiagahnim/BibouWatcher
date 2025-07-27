pub struct BibouHandler {
    inactivitytimer: usize
}

impl Default for BibouHandler {
    fn default() -> Self {
        BibouHandler { inactivitytimer: 0 }
    }
}