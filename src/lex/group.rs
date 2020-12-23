pub trait Group {
    fn match_with(&self, text: &[char], offset: usize) -> Option<u32>;
    fn contains_any(&self) -> bool;
    fn name(&self) -> String;
    fn min_matches(&self) -> usize;
    fn render(&self) -> String;
}
