use crate::server::response::Response;

pub fn handle_get(path: &str) -> Response {
    let mut response = Response {
        status: 200,
        messgage: String::from("OK"),
        content_length: 0,
        body: String::from("")
    };
    match path {
        "/" => {
            response.body = String::from("Hello I am home page");
            response.content_length = response.body.len();

            return response;
        },
        "/about" => {
            // Get the html file content from assets/index.html and assign it to response.body
            let file_content = std::fs::read_to_string("assets/index.html").unwrap();

            response.body = file_content;
            response.content_length = response.body.len();

            return response;
        },
        _ => {
            response.status = 404;
            response.messgage = String::from("Not Found");
            response.body = String::from("404 Not Found");
            response.content_length = response.body.len();

            return response;
        }
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_get_home() {
        let path = "/";
        let response = handle_get(path);

        assert_eq!(response.status, 200);
        assert_eq!(response.messgage, "OK");
        assert_eq!(response.content_length, 20);
        assert_eq!(response.body, "Hello I am home page");
    }

    #[test]
    fn test_handle_get_about() {
        let path = "/about";
        let response = handle_get(path);

        assert_eq!(response.status, 200);
        assert_eq!(response.messgage, "OK");
        assert_eq!(response.body, "<h1>Rust Server</h1>\n<p>Server is running...</p>\n\n<ul>\n  <li>This is a simple web server written in Rust.</li>\n  <li>The server is running on port <code>8080</code>.</li>\n</ul>\n");
        assert_eq!(response.content_length, 177);
    }

    #[test]
    fn test_handle_get_not_found() {
        let path = "/not_found";
        let response = handle_get(path);

        assert_eq!(response.status, 404);
        assert_eq!(response.messgage, "Not Found");
        assert_eq!(response.content_length, 13);
        assert_eq!(response.body, "404 Not Found");
    }
}

// To run the tests, run the command `cargo test` in the terminal.