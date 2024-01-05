#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn reset_cursor(&mut self) {
        self.cursor = 0
    }

    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    pub fn transform<T>(&mut self, cb: impl FnOnce(&char) -> Option<T>) -> Option<T> {
        match self.characters.get(self.cursor) {
            Some(input) => match cb(input) {
                Some(output) => {
                    self.cursor += 1;
                    Some(output)
                }
                None => None,
            },
            None => None,
        }
    }

    pub fn scan<T>(&mut self, mut cb: impl FnMut(&char) -> Option<T>) -> Option<T> {
        loop {
            if self.is_done() {
                return None;
            }
            match self.characters.get(self.cursor) {
                Some(input) => match cb(input) {
                    Some(output) => {
                        self.cursor += 1;
                        Some(output)
                    }
                    None => None,
                },
                None => None,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, ops::Add};

    #[test]
    fn test_transform() {
        //let content = fs::read_to_string("src/bin/input1.txt").unwrap();
        let content = fs::read_to_string("src/bin/test_input.txt").unwrap();
        let lines: Vec<&str> = content.lines().collect();

        let mut scanner = Scanner::new(lines[0]);
        assert_eq!(scanner.cursor, 0);

        let first_char = scanner.transform(|character| Some(character.clone()));

        //assert_eq!(games_list.id_sum, 2563);
        assert_eq!(first_char.unwrap(), '4');
        assert_eq!(scanner.cursor, 1);

        scanner.reset_cursor();

        assert_eq!(scanner.cursor, 0);
    }

    #[test]
    fn test_transform_line() {
        //let content = fs::read_to_string("src/bin/input1.txt").unwrap();
        let content = fs::read_to_string("src/bin/test_input.txt").unwrap();
        let lines: Vec<&str> = content.lines().collect();

        let mut scanner = Scanner::new(lines[0]);
        let mut line_string = String::new();

        while !scanner.is_done() {
            let curr_char = scanner.transform(|character| Some(character.clone()));
            //
            match curr_char {
                Some(element) => line_string.push(element),
                None => continue,
            };
        }

        assert_eq!(line_string, "467..114..".to_string());
    }
}
