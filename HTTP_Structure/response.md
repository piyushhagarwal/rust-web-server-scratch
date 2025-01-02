The structure of an HTTP response is similar to an HTTP request but serves the opposite role: it sends data from the server to the client. Understanding its structure is crucial when building an HTTP server.

---

### **Components of an HTTP Response**

An HTTP response consists of the following parts:

1. **Status Line**
2. **Headers**
3. **Empty Line**
4. **Message Body (Optional)**

Each component is separated by `\r\n` (carriage return and line feed).

---

### **1. Status Line**

The status line is the first line of the HTTP response. It provides the client with the outcome of its request.

**Structure:**

```
<HTTP Version> <Status Code> <Reason Phrase>\r\n
```

**Example:**

```
HTTP/1.1 200 OK\r\n
```

**Explanation:**

1. **HTTP Version:**  
   The version of the HTTP protocol used by the server (e.g., `HTTP/1.1`, `HTTP/2.0`).

2. **Status Code:**  
   A three-digit numeric code indicating the result of the request. Common status codes include:

   - `200 OK`: The request succeeded.
   - `404 Not Found`: The requested resource was not found.
   - `500 Internal Server Error`: The server encountered an error.

3. **Reason Phrase:**  
   A short description of the status code, intended for human readability. For example:
   - `OK` for `200`.
   - `Not Found` for `404`.

---

### **2. Headers**

Headers provide additional metadata about the response. Each header is written as a key-value pair, followed by `\r\n`.

**Structure:**

```
Header-Name: Header-Value\r\n
```

**Example:**

```
Content-Type: text/html\r\n
Content-Length: 1234\r\n
Connection: close\r\n
```

**Common Response Headers:**

1. **Content-Type:**  
   Indicates the media type of the response body (e.g., `text/html`, `application/json`).

2. **Content-Length:**  
   Specifies the size of the response body in bytes.

3. **Connection:**  
   Controls whether the server keeps the connection open or closes it after the response (`keep-alive` or `close`).

4. **Server:**  
   Identifies the server software (e.g., `nginx/1.21.0`).

5. **Date:**  
   Provides the date and time when the response was generated.

---

### **3. Empty Line**

An empty line (just `\r\n`) separates the headers from the optional message body. It serves as a delimiter to mark the end of the headers section.

---

### **4. Message Body (Optional)**

The message body contains the actual data requested by the client. Its format is specified by the `Content-Type` header. The body can include:

- HTML content for webpages.
- JSON for API responses.
- Binary data for files or images.

**Example:**

- **HTML:**
  ```
  <html>
  <head><title>Example</title></head>
  <body><h1>Hello, World!</h1></body>
  </html>
  ```
- **JSON:**
  ```
  {"message": "Success", "data": []}
  ```

The body does not end with `\r\n`; its size is defined by the `Content-Length` header.

---

### **Full Example: HTTP Response with `\r\n`**

Here’s a complete HTTP response:

```
HTTP/1.1 200 OK\r\n
Content-Type: text/html\r\n
Content-Length: 135\r\n
Connection: close\r\n
\r\n
<html>
<head><title>Welcome</title></head>
<body><h1>Hello, world!</h1></body>
</html>
```

**Breakdown:**

1. **Status Line:**

   ```
   HTTP/1.1 200 OK\r\n
   ```

   - HTTP Version: `HTTP/1.1`
   - Status Code: `200`
   - Reason Phrase: `OK`

2. **Headers:**

   ```
   Content-Type: text/html\r\n
   Content-Length: 135\r\n
   Connection: close\r\n
   ```

   - `Content-Type`: Indicates the response body is HTML.
   - `Content-Length`: Specifies the length of the body (135 bytes).
   - `Connection`: Instructs the client that the server will close the connection after sending the response.

3. **Empty Line:**

   ```
   \r\n
   ```

   Separates the headers from the body.

4. **Message Body:**
   ```
   <html>
   <head><title>Welcome</title></head>
   <body><h1>Hello, world!</h1></body>
   </html>
   ```

---

### **HTTP Status Codes in Detail**

#### **1xx: Informational**

- `100 Continue`: The client can continue sending the request body.
- `101 Switching Protocols`: The server is switching protocols as requested by the client.

#### **2xx: Success**

- `200 OK`: The request succeeded.
- `201 Created`: A new resource has been created.
- `204 No Content`: The request succeeded but there is no content to return.

#### **3xx: Redirection**

- `301 Moved Permanently`: The resource has been permanently moved.
- `302 Found`: The resource is temporarily located at a different URL.
- `304 Not Modified`: The resource hasn't changed; use the cached version.

#### **4xx: Client Errors**

- `400 Bad Request`: The request is malformed.
- `401 Unauthorized`: Authentication is required.
- `403 Forbidden`: The client does not have permission.
- `404 Not Found`: The resource cannot be found.

#### **5xx: Server Errors**

- `500 Internal Server Error`: A generic server error.
- `502 Bad Gateway`: The server received an invalid response from an upstream server.
- `503 Service Unavailable`: The server is temporarily overloaded or under maintenance.

---

### **Key Takeaways**

1. The **status line** communicates the result of the request.
2. **Headers** provide metadata about the response, such as content type and size.
3. The **empty line** separates the headers from the body.
4. The **message body** contains the requested data or error details.
