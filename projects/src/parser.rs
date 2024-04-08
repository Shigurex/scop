use std::fs;

use anyhow::Result;

pub trait Parser<T> {
    fn parse(&self, file: &str) -> Result<T> {
        let contents = self.read_file_to_string(file)?;
        let contents_without_comments = self.remove_comments(contents);
        let contents_vectored = self.vectorize_contents(contents_without_comments);
        self.format(contents_vectored)
    }

    fn read_file_to_string(&self, path: &str) -> Result<String> {
        let contents = fs::read_to_string(path)?;
        Ok(contents)
    }

    fn remove_comments(&self, contents: String) -> Vec<String> {
        contents
            .lines()
            .map(|line| match line.find('#') {
                Some(index) => String::from(&line[0..index]),
                None => String::from(line),
            })
            .collect()
    }

    fn vectorize_contents(contents: Vec<String>) -> Vec<Vec<&str>> {
        contents
            .iter()
            .map(|line| line.split_whitespace().collect::<Vec<&str>>())
            .filter(|line| !line.is_empty())
            .collect()
    }

    fn format(&self, contents: Vec<Vec<String>>) -> Result<T>;
}