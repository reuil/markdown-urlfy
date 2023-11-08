use html_escape::decode_html_entities;
use regex::Regex;
use std::error::Error;
use std::io;

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().to_string()
}

fn get_html(url: &str) -> Result<String, Box<dyn Error>> {
    use encoding_rs::{SHIFT_JIS, UTF_8};
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent("markdown-urlfy (github.com/reuil/markdown-urlfy)")
        .build()
        .unwrap_or_else(|err| panic!("Failed to build client: {:?}", err));
    let response = client
        .get(url)
        .send()
        .unwrap_or_else(|err| panic!("Failed to get: {}\nReason: {:?}", url, err));
    let bytes = response.bytes()?;
    if bytes.is_empty() {
        panic!("Failed to get: {}\nReason: Empty response", url);
    }
    let body_string = String::from_utf8_lossy(&bytes);
    let shift_jis_regex =
        Regex::new(r#"charset=["']?((shift|S(hift|HIFT))_(jis|J(is|IS)))["']?"#).unwrap();
    let encoding = if shift_jis_regex.is_match(&body_string) {
        SHIFT_JIS
    } else {
        UTF_8
    };
    let (decoded_string, _, _) = encoding.decode(&bytes);
    let decoded_string = decode_html_entities(&decoded_string);
    Ok(decoded_string.to_string())
}

fn get_title(url: &str) -> Result<String, Box<dyn Error>> {
    let html = get_html(url)?;
    let title_regex = Regex::new(r"<title>(.*)</title>").unwrap();
    let title = title_regex
        .captures(&html)
        .unwrap_or_else(|| {
            panic!(
                "Failed to get title from: {}\n Reason: Maybe invalid html",
                url
            )
        })
        .get(1)
        .unwrap_or_else(|| {
            panic!(
                "Failed to get title from: {}\n Reason: Maybe there is no title tag",
                url
            )
        })
        .as_str();
    Ok(title.to_string())
}

fn replace_url_with_markdown_format(text: &str) -> String {
    let url_regex = Regex::new(r"(\[\]\()?https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.,~#?&//=]*)\)?")
        .unwrap();
    let mut replaced_text = text.to_string();
    url_regex.find_iter(text).for_each(|m| {
        let url = m.as_str().trim_start_matches("[](").trim_end_matches(")"); // Get the part surrounded by []()
        let title = get_title(url).unwrap_or("".to_string());
        let title = title.trim();
        let markdown_format = format!("[{}]({})", title, url);
        replaced_text = replaced_text.replace(m.as_str(), &markdown_format);
    });
    replaced_text
}

fn main() {
    let buffer = read_buffer();
    println!("{}", replace_url_with_markdown_format(&buffer));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replace_url_with_markdown_format_test() {
        let text = "これは https://reuil.github.io/misc/utf_8_test_page.html です。これは [](https://reuil.github.io/misc/shift_jis_test_page.html)です。";
        let replaced_text = replace_url_with_markdown_format(text);
        assert_eq!(replaced_text, "これは [utf-8で書かれたタイトル](https://reuil.github.io/misc/utf_8_test_page.html) です。これは [shift_jisで書かれたタイトル](https://reuil.github.io/misc/shift_jis_test_page.html)です。");
    }

    #[test]
    fn get_title_test() {
        let url = "https://reuil.github.io/misc/utf_8_test_page.html";
        let title = get_title(url).unwrap();
        assert_eq!(title, "utf-8で書かれたタイトル");
        let url = "https://reuil.github.io/misc/shift_jis_test_page.html";
        let title = get_title(url).unwrap();
        assert_eq!(title, "shift_jisで書かれたタイトル");
    }
}
