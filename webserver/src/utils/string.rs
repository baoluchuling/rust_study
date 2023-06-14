pub struct StringExtension {}

impl StringExtension {
    pub fn split(text: String, sep: &str) -> Vec<String> {
        let contents = text.split(sep);

        let mut result: Vec<String> = Vec::new();
        for content in contents {
            result.push(content.to_string());
        }

        return result;
    }
}
