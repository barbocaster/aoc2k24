use std::fs;

struct Scanner {
    sum: u32,
    text: Vec<char>,
    position: usize
}

impl Scanner {
    fn new(input: &str) -> Self {
        let text = fs::read_to_string(input).unwrap().chars().collect();
        let position = 0;
        let sum = 0;
        Scanner { text, position, sum } 
    }

    fn identifier(&mut self) -> String {

        let mut new_string = String::new();

        let mut get_next = self.peek();

        while self.text[get_next].is_alphabetic() && self.at_end() != true {
            new_string.push_str(self.text[get_next].to_string().as_str());
            self.advance();
            get_next = self.peek();
        }

        new_string 
    }

    fn get_number(&mut self) -> String {
        let mut new_string = String::new();

        let mut get_next = self.peek();

        while self.text[get_next].is_numeric() && self.at_end() != true {
            new_string.push_str(self.text[get_next].to_string().as_str());
            self.advance();
            get_next = self.peek();
        }

        new_string
    }

    fn string(&mut self) {

        if !self.identifier().contains("mul") {
            return
        }
    
        if !self.make_match("(") {
            return
        };

        self.advance();

        let factor1: u32 = match self.get_number().parse() {
            Ok(x) => x,
            Err(_) => return,
        };

        println!("{}", factor1);
        if !self.make_match(",") {
            return
        };

        self.advance();

        let factor2: u32 = match self.get_number().parse() {
            Ok(x) => x,
            Err(_) => return,
        };
        println!("{}", factor2);

        if !self.make_match(")") {
            return
        };
        
        self.sum = self.sum + (factor1 * factor2);
    }

    fn make_match(&self, is_match: &str) -> bool {
        let get_next = self.peek();
        if self.text[get_next].to_string().as_str() == is_match {
            return true;
        }
    
        return false
    }

    fn at_end(&self) -> bool {
        self.position >= self.text.len() - 1
        
    }
    fn advance(&mut self) {
        self.position += 1;
    }

    fn peek(&self) -> usize {
        self.position - 1
    }
}

fn main() {

    // I DONT KNOW REGEX SO HERES A COOL LEXER-SCANNER
    
    let mut scanner = Scanner::new("sample2.txt");

    loop {
        if scanner.at_end() {
            break;
        }

        scanner.advance();

        let character = scanner.text[scanner.position];

        match character {
            'a'..'z' => scanner.string(),
            _ => continue
        };
    }

    println!("{}", scanner.sum);
}
