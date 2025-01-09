pub struct Response {
    pub status: u16,
    pub messgage: String,
    pub content_length: usize,
    pub body: String,
}