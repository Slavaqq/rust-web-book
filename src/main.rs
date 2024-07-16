use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, title:  {}, content: {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}", self.0)
    }
}

impl std::str::FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidData, "No id provided")),
        }
    }
}

fn main() {
    let question = Question::new(
        QuestionId::from_str("1").expect("Id not provided!"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{:#?}", question);
}
