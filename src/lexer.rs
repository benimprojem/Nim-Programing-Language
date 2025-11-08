// src/lexer.rs

use crate::token::{Token, lookup_ident};

pub struct Lexer {
    input: String,
    position: usize,      // Mevcut karakterin indeksi
    read_position: usize, // Sonraki karakterin indeksi
    ch: char,             // Mevcut karakter
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    // Boşlukları (whitespace) atlar.
    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    // Harf veya alt çizgi ile başlayan tanımlayıcıları okur.
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
    
    // Tırnak işaretleri arasındaki dizeyi okur
    fn read_string(&mut self) -> String {
        // İlk tırnak işaretini atla
        self.read_char(); 
        let position = self.position;

        // Kapatma tırnağına veya EOF'a kadar oku
        while self.ch != '"' && self.ch != '\0' {
            // Kaçış karakterlerini (örneğin \n, \t) burada ele almak gerekir
            if self.ch == '\\' {
                // Kaçış karakterini atla ve sonraki karakteri oku (basit uygulama)
                self.read_char(); 
            }
            self.read_char();
        }

        let s = self.input[position..self.position].to_string();
        
        // Kapatma tırnağını atla
        if self.ch == '"' {
            self.read_char();
        }

        s
    }

    // Sayıları (tamsayı veya kayan nokta) okur
    fn read_number(&mut self) -> Token {
        let start_pos = self.position;
        let mut is_float = false;

        // Tamsayı kısmını oku
        while self.ch.is_digit(10) { self.read_char(); }

        // Ondalık kısmı kontrol et
        if self.ch == '.' {
            is_float = true;
            self.read_char();
            while self.ch.is_digit(10) { self.read_char(); }
        }

        // Bilimsel gösterim (e/E) kontrolü (BÖLÜM 2.3)
        if self.ch == 'e' || self.ch == 'E' {
            is_float = true;
            self.read_char();
            if self.ch == '+' || self.ch == '-' { self.read_char(); } // İşareti oku
            while self.ch.is_digit(10) { self.read_char(); } // Üs değerini oku
        }

        let literal_str = &self.input[start_pos..self.position];
        
        // Tamsayı veya Kayan Nokta olarak ayrıştır
        if is_float {
            match literal_str.parse::<f64>() {
                Ok(val) => Token::Float(val), // Düzeltildi
                Err(_) => Token::Illegal,
            }
        } else {
            match literal_str.parse::<i64>() {
                Ok(val) => Token::Int(val), // Düzeltildi
                Err(_) => Token::Illegal,
            }
        }
    }


    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '\0' => Token::EOF,
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char(); 
                    Token::Equal // Düzeltildi (==)
                } else {
                    Token::Assign // Düzeltildi (=)
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char(); 
                    Token::NotEqual // Düzeltildi (!=)
                } else {
                    Token::Bang // (!)
                }
            }
            '+' => {
                if self.peek_char() == '=' {
                    self.read_char(); 
                    Token::PlusAssign // Düzeltildi (+=)
                } else {
                    Token::Plus // (+)
                }
            }
            '-' => {
                if self.peek_char() == '=' {
                    self.read_char(); 
                    Token::MinusAssign // Düzeltildi (-=)
                } else if self.peek_char() == '>' {
                    self.read_char();
                    Token::Arrow // (->)
                } else {
                    Token::Minus // (-)
                }
            }
            '*' => Token::Star, // Düzeltildi (*)
            '/' => Token::Slash,
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char(); 
                    Token::LessEqual
                } else {
                    Token::LessThan
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char(); 
                    Token::GreaterEqual
                } else {
                    Token::GreaterThan
                }
            }
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ':' => Token::Colon,
            '.' => Token::Dot,
            '%' => Token::Modulo,
            '&' => Token::Ampersand,
            '~' => Token::Tilde,
            '^' => Token::Caret,
            '|' => Token::VBar,
            '$' => Token::Dollar,
            
            // String Literali
            '"' => {
                let s = self.read_string();
                return Token::String(s); // Düzeltildi
            }

            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    let literal = self.read_identifier();
                    return lookup_ident(&literal); // Anahtar kelime veya Tanımlayıcıyı çöz
                } else if self.ch.is_digit(10) {
                    return self.read_number(); // Sayısal Litera
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        token
    }

    // Bir sonraki karakteri ilerlemeden kontrol eder.
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }
}

