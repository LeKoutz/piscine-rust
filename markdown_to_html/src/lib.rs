pub fn markdown_to_html(s: &str) -> String {
    let lines: Vec<String> = s.lines().map(convert_line).collect();
    let mut html = convert_inline(&lines.join("\n"));
    if lines.last().map_or(false, |line| line.trim_start().starts_with("<h")) {
        html.push('\n');
    }
    html
}

fn convert_line(line: &str) -> String {
    let content = line.trim_start();
    let indent = &line[..line.len() - content.len()];

    let prefixes = [("### ", "h3"), ("## ", "h2"), ("# ", "h1"), ("> ", "blockquote")];
    for (prefix, tag) in prefixes {
        if let Some(rest) = content.strip_prefix(prefix) {
            return format!("{indent}<{tag}>{rest}</{tag}>");
        }
    }
    line.to_string()
}

fn convert_inline(text: &str) -> String {
    let mut result = String::new();
    let mut bold = false;
    let mut italic = false;
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '*' {
            if chars.peek() == Some(&'*') {
                chars.next();
                result.push_str(if bold { "</strong>" } else { "<strong>" });
                bold = !bold;
            } else {
                result.push_str(if italic { "</em>" } else { "<em>" });
                italic = !italic;
            }
        } else {
            result.push(c);
        }
    }
    result
}