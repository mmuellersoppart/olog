pub struct Asker {
    questions: Vec<Question>,
    current_question: usize,
    answers: Vec<String>,
}

struct Question {
    id: u16,
    question: String,
}

impl Question {
    fn new(id: u16, question: String) -> Question {
        Question {
            id,
            question,
        }
    }
}

impl Asker {

    pub fn new() -> Self {
        Asker {
            questions: vec![],
            current_question: 0,
            answers: vec![],
        }
    }

    pub fn question(mut self, q: String) -> Self {
        let id = self.questions.len();
        self.questions.push(Question::new(id.try_into().unwrap(), q));
        self
    }

    pub fn question_count(self) -> u16 {
        return self.questions.len() as u16
    }

    pub fn set_current_question(&mut self, question_id: usize) {
        self.current_question = question_id
    }

    pub fn csv_transcript(self) -> u16 {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::asker::Asker;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn register_questions() {
        let asker = Asker::new()
                        .question("Organization".to_string())
                        .question("Project".to_string());

        assert_eq!(asker.question_count(), 2)
    }

    #[test]
    fn set_current_question() {
        let mut asker = Asker::new()
            .question("Organization".to_string())
            .question("Project".to_string());

        asker.set_current_question(4);

        assert_eq!(asker.current_question, 4)
    }
}
