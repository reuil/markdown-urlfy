use std::io;

fn get_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let bytes = response.bytes()?;
    let body_string = String::from_utf8_lossy(&bytes);
    let shift_jis_regex =
        regex::Regex::new(r#"charset=["']?((shift|S(hift|HIFT))_(jis|J(is|IS)))["']?"#).unwrap();
    let encoding =
        if shift_jis_regex.is_match(&body_string) {
            encoding_rs::SHIFT_JIS
        } else {
            encoding_rs::UTF_8
        };
    let (decoded_string, _, _) = encoding.decode(&bytes);
    Ok(decoded_string.to_string())
}

fn get_title(url: &str) -> Option<String> {
    let body = get_html(url).unwrap();
    let title_start_index = body.find("<title>").unwrap_or(0) + 7;
    let title_end_index = body.find("</title>").unwrap_or(0);
    if title_start_index == 6 || title_end_index == 0 {
        return None;
    }
    let title = &body[title_start_index..title_end_index];
    Some(title.to_string())
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().to_string()
}

fn replace_url_with_markdown_format(text: &str) -> String {
    let url_regex = regex::Regex::new(r"(\[\]\()?https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.,~#?&//=]*)\)?")
        .unwrap();
    let mut replaced_text = text.to_string();
    url_regex.find_iter(text).for_each(|m| {
        let url = m.as_str().trim_start_matches("[](").trim_end_matches(")"); // [](URL)で囲まれた部分を取得
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
        let text = "[](https://example.com/)は、とても良いサイトです。Rustについては、本家HPがあります。https://www.rust-lang.org/ とても良く出来ています。";
        let replaced_text = replace_url_with_markdown_format(text);
        assert_eq!(replaced_text, "[Example Domain](https://example.com/)は、とても良いサイトです。Rustについては、本家HPがあります。[Rust Programming Language](https://www.rust-lang.org/) とても良く出来ています。");
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
