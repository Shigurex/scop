use std::fs;

use anyhow::Result;

pub trait Parser<T> {
    fn parse(file: &str) -> Result<T> {
        let contents = Self::read_file_to_string(file)?;
        let contents_without_comments = Self::remove_comments(contents);
        let contents_vectored = Self::vectorize_contents(contents_without_comments);
        Self::format(contents_vectored)
    }

    fn read_file_to_string(path: &str) -> Result<String> {
        let contents = fs::read_to_string(path)?;
        Ok(contents)
    }

    fn remove_comments(contents: String) -> Vec<String> {
        contents
            .lines()
            .map(|line| match line.find('#') {
                Some(index) => String::from(&line[0..index]),
                None => String::from(line),
            })
            .collect()
    }

    fn vectorize_contents(contents: Vec<String>) -> Vec<Vec<String>> {
        contents
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .filter(|line| !line.is_empty())
            .collect()
    }

    fn format(contents: Vec<Vec<String>>) -> Result<T>;
}
