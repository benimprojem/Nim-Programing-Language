// src/parser.rs
use crate::token::{Token, Lexer};
use crate::ast::{Node, Type, Parameter, Argument};

// --- İfade Önceliği (Pratt Parser) ---
// Öncelik Seviyeleri (E0599 Fix: Comparison ve Member eklendi)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Assignment,     // =
    LogicalOr,      // or
    LogicalAnd,     // and
    Comparison,     // ==, !=, <, >, <=, >= (NEW)
    Sum,            // +, -
    Product,        // *, /
    Prefix,         // -X, !X, *X, &X
    Call,           // fn()
    Index,          // arr[idx]
    Member,         // struct.member (NEW)
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token, // Bir sonraki token
    errors: Vec<ParserError>, // E0599 Fix: add_error için gerekli
}

// Hata Tipi Tanımlaması (E0599 Fix: add_error metodu için gerekli)
#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
         // E0599 Fix: errors alanını başlat
        Parser { lexer, current_token, peek_token, errors: Vec::new() }
    }
	
	// Hata Ekleme Fonksiyonu (E0599 Fix: Missing add_error)
    fn add_error(&mut self, message: String) {
        self.errors.push(ParserError { message });
    }
	
    // İleri hareket et ve tokenları güncelle
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    // Beklenen bir token ile mevcut token'ı karşılaştırır
    fn expect_peek(&mut self, t: &Token) -> bool {
        if self.peek_token == *t {
            self.next_token();
            true
        } else {
            // Hata üretme (SyntaxError)
            false
        }
    }

    // Mevcut token'ın öncelik seviyesini döndürür.
    fn peek_precedence(&self) -> Precedence {
        self.get_precedence(&self.peek_token)
    }

    // Token'a göre öncelik seviyesini belirler.
	fn get_precedence(&self, token: &Token) -> Precedence {
        match token {
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Star | Token::Slash => Precedence::Product,
            // E0599 Fix: Comparison eklendi
            Token::Equal | Token::NotEqual | Token::LessThan | Token::GreaterThan | Token::LessEqual | Token::GreaterEqual => Precedence::Comparison,
            // E0599 Fix: Member eklendi
            Token::Dot => Precedence::Member, 
            Token::LParen => Precedence::Call,
            Token::LBracket => Precedence::Index,
            Token::Assign | Token::PlusAssign | Token::MinusAssign | Token::StarAssign | Token::SlashAssign => Precedence::Assignment,
            _ => Precedence::Lowest,
        }
    }
    
    // Programın giriş noktası (Program düğümünü döndürür)
    pub fn parse_program(&mut self) -> Node {
        let mut statements = Vec::new();

        while self.current_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token(); // Hata olsa bile ilerle
        }
        
        // Hataları döndürmek için bir mekanizma eklenmeli
        if !self.errors.is_empty() {
            eprintln!("Ayrıştırma Hataları: {:?}", self.errors);
        }

        Node::Program(statements)
    }
	
	// İkili Operatör Kontrolü: Atama operatörü mü?
    fn is_assignment_operator(operator: &Token) -> bool {
        // Düzeltildi: PlusAssign, MinusAssign
        if operator == &Token::Assign || operator == &Token::PlusAssign || operator == &Token::MinusAssign {
            true
        } else {
            false
        }
    }
	
    // İkili (Infix) İfade Ayrıştırma
    fn parse_infix_expression(&mut self, left: Node) -> Option<Node> {
        let operator = self.current_token.clone();
        let precedence = self.get_precedence(&operator);
        self.next_token(); // Operatör tüketildi.

        let right = self.parse_expression_with_precedence(precedence)?; // Aynı veya daha yüksek önceliklerle sağ tarafı ayrıştır.

        // Atama operatörleri için özel durum (Assignment)
        if operator == Token::Assign || operator == Token::PlusAssign || operator == Token::MinusAssign {
            Some(Node::Assignment {
                target: Box::new(left),
                operator,
                value: Box::new(right),
            })
        } else {
            // Normal İkili Operatör (BinaryExpression)
            Some(Node::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            })
        }
    }

    // Basit Değişken Kullanımı
    fn parse_identifier_expression(&mut self) -> Option<Node> {
        if let Token::Identifier(name) = self.current_token.clone() {
            self.next_token(); // Identifier tüketildi.
            Some(Node::Identifier(name))
        } else {
            None
        }
    }

    // Sabit Değerler (10, "metin", true, null)
    fn parse_literal(&mut self) -> Option<Node> {
        let literal_token = self.current_token.clone();
        self.next_token(); // Literal tüketildi.
        
        Some(Node::Literal(literal_token))
    }
	// Literal değerleri (Sayı, String, Boolean, Null) ayrıştırır.
	fn parse_literal_expression(&mut self) -> Option<Node> {
		let token = self.current_token.clone();
		self.next_token();
		
		match token {
			// Düzeltildi: Int, Float, String
			Token::Int(value) => Some(Node::IntegerLiteral(value)),
			Token::Float(value) => Some(Node::FloatLiteral(value)),
			Token::String(value) => Some(Node::StringLiteral(value)),
			Token::True => Some(Node::BooleanLiteral(true)),
			Token::False => Some(Node::BooleanLiteral(false)),
			Token::Null => Some(Node::NullLiteral),
			_ => { 
				self.add_error(format!("Beklenmeyen literal token: {:?}", token));
				None 
			}
		}
	}
    // Parantezli İfade: (a + b)
    fn parse_grouped_expression(&mut self) -> Option<Node> {
        // Current token: LParen ( ( )
        self.next_token(); // '(' tüketildi.

        let exp = self.parse_expression()?; // İçerideki ifadeyi ayrıştır.

        if !self.expect_peek(&Token::RParen) {
            // Hata: Parantezli ifade ')' ile kapatılmalıdır.
            return None;
        }
        
        Some(exp) // Parantezler AST'de yer almaz.
    }


    // İfade Ayrıştırmanın Merkezi
    fn parse_expression(&mut self) -> Option<Node> {
        self.parse_expression_with_precedence(Precedence::Lowest)
    }

    // parse_expression'ın basitleştirilmiş bir taslağı
	fn parse_expression_with_precedence(&mut self, precedence: Precedence) -> Option<Node> {
		let mut left_exp = match self.current_token {
			// Düzeltildi: Star
			Token::Star | Token::Ampersand | Token::Bang | Token::Minus => self.parse_prefix_expression()?,
			
			// Düzeltildi: Int, Float, String
			Token::Int(_) | Token::Float(_) | Token::String(_) | Token::True | Token::False | Token::Null => self.parse_literal_expression()?,

			Token::Identifier(_) => self.parse_identifier_expression()?,
			Token::LParen => self.parse_grouped_expression()?,
			// ... Diğer başlangıç ifadeleri (Literal, Parenthesis, etc.)
			_ => {
                self.add_error(format!("Beklenmeyen başlangıç token'ı: {:?}", self.current_token));
                return None
            },
		};
		
		
		
		while self.peek_token != Token::Semicolon && precedence < self.peek_precedence() {
			match self.peek_token {
                // Düzeltildi: Equal, NotEqual, Star
				Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Equal | Token::NotEqual => {
					// İkili Operatör
					self.next_token(); // Operatörü tüket
					let operator = self.current_token.clone();
					self.next_token(); // Next'i tüket
					
					let next_precedence = self.get_precedence(&operator);
					let right_exp = self.parse_expression_with_precedence(next_precedence)?;
					
					left_exp = Node::BinaryExpression {
						operator,
						left: Box::new(left_exp),
						right: Box::new(right_exp),
					};
				},
				// ... (Diğer postfix operatörler)
				_ => return Some(left_exp),
			}
		}

		Some(left_exp)
	}

    // --- TEMEL DEYİM AYRIŞTIRICI DISPATCHER (parse_statement) ---
    fn parse_statement(&mut self) -> Option<Node> {
        // 1. Özel Etiketli Bloklar (fast_exec: TEST { ... })
        if let Token::Identifier(ref ident) = self.current_token {
            if ident == "fast_exec" && self.peek_token == Token::Colon {
                return self.parse_fast_exec_block();
            }
        }

        match self.current_token.clone() {
            // Tanımlamalar
            Token::KeywordVar | Token::KeywordConst => self.parse_declaration(),
            Token::KeywordFn | Token::KeywordInline => self.parse_function_declaration(),
            Token::KeywordGroup => self.parse_group_declaration(),
            Token::KeywordStruct => self.parse_struct_declaration(),
            Token::KeywordExtend => self.parse_extend_block(),
            Token::KeywordTypedef => self.parse_typedef_declaration(),
            Token::KeywordEnum => self.parse_enum_declaration(),

            // Akış Kontrolü
            Token::KeywordIf => self.parse_if_statement(),
            Token::KeywordVar | Token::KeywordConst => self.parse_declaration_statement(),
			// Identifier ile başlayan durumlar (Atama, Çağrı, Etiketli Blok)
            Token::Identifier(_) => self.parse_expression_statement(),
            _ => {
                self.add_error(format!("Beklenmeyen başlangıç token'ı: {:?}", self.current_token)); // E0599 Fix: add_error
                None
            },
            Token::KeywordWhile => self.parse_while_loop(),
            Token::KeywordFor => self.parse_for_loop(),
            Token::KeywordBreak | Token::KeywordContinue | Token::KeywordReturn => self.parse_control_flow(),
            Token::KeywordRolling => self.parse_rolling_statement(),
            
            // Modül Yönetimi
            Token::KeywordUse => self.parse_use_statement(),
            Token::KeywordExport => self.parse_export_statement(),
            Token::Colon => self.parse_module_prefix(), // :modül_adı;
            
            // Blok Başlangıcı
            Token::LBrace => self.parse_block_statement().map(|b| *b),

            _ => self.parse_expression_statement(), // Diğer ifadeler (tek başına ifade, match expression vb.)
        }
    }
    
    // Basit ifadeyi deyime çevirir (expr;)
    fn parse_expression_statement(&mut self) -> Option<Node> {
        let expression = self.parse_expression_with_precedence(Precedence::Lowest)?;
        
        if self.peek_token == Token::Semicolon {
            self.next_token();
        }
        
        Some(expression) // Şimdilik sadece ifadeyi döndürüyor, gerçekte bir ExpressionStatement olmalı.
    }
	// Değişken Tanımlama
    fn parse_declaration_statement(&mut self) -> Option<Node> {
        // ... (body implementation omitted for brevity) ...
        Some(Node::NoOp)
    }


    // --- DEYİM AYRIŞTIRICILARI ---

    // var x: i32 = 10;
    fn parse_declaration(&mut self) -> Option<Node> {
        let is_const = self.current_token == Token::KeywordConst;
        self.next_token(); // 'var' veya 'const' tüket

        let mut names = Vec::new();
        let name = match self.current_token.clone() {
            Token::Identifier(name) => { self.next_token(); name },
            _ => { return None; } // Hata: İsim bekleniyor
        };
        names.push(name);
        
        // Tip Notu Kontrolü: : i32
        let type_hint = if self.current_token == Token::Colon {
            self.next_token(); // ':' tüket
            self.parse_type()
        } else {
            None
        };

        // Başlatıcı Kontrolü: = value
        let initializer = if self.current_token == Token::Assign {
            self.next_token(); // '=' tüket
            self.parse_expression().map(Box::new) // Box<Node> olarak sarıldı
        } else {
            None
        };
        
        if self.current_token == Token::Semicolon { self.next_token(); }

        Some(Node::Declaration { 
            is_const, 
            names, 
            type_hint, 
            initializer, 
        })
    }
    
    // Bir BlockStatement'ı ayrıştırır ({ ... })
    fn parse_block_statement(&mut self) -> Option<Box<Node>> {
        // ... (body implementation omitted, but assumed to return Option<Box<Node>> to fix E0308)
        
        // Bu fonksiyonun Option<Box<Node>> döndürmesi, aşağıdaki E0308 hatasını çözer.
        // Hata ayıklama için geçici olarak Box::new(Node::NoOp) döndürelim:
        Some(Box::new(Node::NoOp))
    }

    // If/Else deyimini ayrıştırır
    fn parse_if_statement(&mut self) -> Option<Node> {
        self.next_token(); // Consume `if`
        
        // 1. Koşul
        if !self.expect_peek(&Token::LParen) { return None; }
        self.next_token(); // Consume `(`
        let condition = self.parse_expression_with_precedence(Precedence::Lowest).map(Box::new)?;
        if !self.expect_peek(&Token::RParen) { return None; }
        
        // 2. Consequence (If bloğu)
        self.next_token(); // Consume `)`
        let consequence = self.parse_block_statement()?; // Assumed to return Option<Box<Node>>
        
        // 3. Alternative (Else if / Else)
        let alternative = if self.peek_token == Token::KeywordElse {
            self.next_token(); // Consume `else`

            // Line 335-338 Fix: Type mismatch E0308
            // parse_block_statement'ın zaten Box<Node> döndürdüğünü varsayarak
            if self.current_token == Token::KeywordIf {
                self.next_token(); // Consume `if`
                self.parse_if_statement().map(Box::new) // parse_if_statement() Option<Node> döndürmeli
            } else {
                self.parse_block_statement() // parse_block_statement() Option<Box<Node>> döndürmeli
            }
        } else {
            None
        };
        
        // Line 344 Fix: Mismatched types (consequence: Box<Box<Node>>)
        // consequence'ın zaten Option<Box<Node>> olduğu ve ? operatörüyle unwrap edildiği varsayılır.
        // Eğer parse_block_statement() Option<Box<Node>> döndürüyorsa, consequence Box<Node> olur.
        // Eğer consequence çift kutulanmışsa (*consequence) ile çözülür.

        // Fix'i uygulayalım: consequence'ın Box<Box<Node>> olduğu ve *consequence ile tek kutuya indirildiği varsayılır.
        Some(Node::IfStatement { condition, consequence:  Box::new(*consequence), alternative }) // E0308/344 Fix: *consequence
    }
    
    // while condition { body }
    fn parse_while_loop(&mut self) -> Option<Node> {
        self.next_token(); // 'while' tüket
        let condition = self.parse_expression().map(Box::new)?;
        let body = self.parse_block_statement().map(Box::new)?;
        
        Some(Node::WhileLoop { condition, body: *body })
    }
    
    // for (init; condition; update) { body } VEYA for (array as val) { body }
    fn parse_for_loop(&mut self) -> Option<Node> {
        self.next_token(); // 'for' tüketildi.

        if !self.expect_peek(&Token::LParen) { return None; } // '(' bekleniyor
        // self.next_token(); // '(' expect_peek içinde tüketildi.

        // 1. Array-as-Val (ForInLoop) kontrolü
        // İlk ifadeyi ayrıştır (array/collection)
        let collection_exp = self.parse_expression()?;

        if self.current_token == Token::KeywordAs {
            // Bu bir ForInLoop (for (arr as val))
            self.next_token(); // 'as' tüketildi.
            
            let iterator_name = if let Node::Identifier(name) = self.parse_identifier_expression()? {
                name
            } else {
                // Hata: 'as' sonrası Identifier bekleniyor.
                return None;
            };

            if !self.expect_peek(&Token::RParen) { return None; } // ')' bekleniyor
            // self.next_token(); // ')' expect_peek içinde tüketildi.

            let body = self.parse_block_statement().map(Box::new)?;

            return Some(Node::ForInLoop {
                collection: Box::new(collection_exp),
                iterator_name,
                body: *body,
            });
        }

        // 2. C-Stili For Döngüsü (for (init; condition; update))
        // İlk ifade 'initial' oldu. Geri sarmak gerekebilir.
        
        // Basitlik için, ilk ifadenin C-stili döngünün 'initial' kısmı olduğunu varsayalım.
        let initial = Some(Box::new(collection_exp));

        if self.current_token != Token::Semicolon { return None; } // Hata: ; bekleniyor
        self.next_token(); // ';' tüketildi.

        // Condition (Opsiyonel)
        let condition = if self.current_token != Token::Semicolon {
            self.parse_expression().map(Box::new)
        } else {
            None
        };

        if self.current_token != Token::Semicolon { return None; } // Hata: ; bekleniyor
        self.next_token(); // ';' tüketildi.

        // Update (Opsiyonel)
        let update = if self.current_token != Token::RParen {
            self.parse_expression().map(Box::new)
        } else {
            None
        };

        if !self.expect_peek(&Token::RParen) { return None; } // ')' bekleniyor
        // self.next_token(); // ')' expect_peek içinde tüketildi.

        let body = self.parse_block_statement().map(Box::new)?;

        Some(Node::ForLoop {
            initial,
            condition,
            update,
            body: *body,
        })
    }

    // break, continue, return [value];
    fn parse_control_flow(&mut self) -> Option<Node> {
        let keyword = self.current_token.clone();
        self.next_token(); // Anahtar kelimeyi (break/continue/return) tüket

        let value = if keyword == Token::KeywordReturn {
            // Return sonrası opsiyonel ifadeyi al
            if self.current_token != Token::Semicolon {
                self.parse_expression().map(Box::new)
            } else {
                None
            }
        } else {
            // Break ve Continue için değer yok
            None
        };

        // İfade ; ile bitmelidir.
        if !self.expect_peek(&Token::Semicolon) { return None; }
        // self.next_token(); // ';' expect_peek içinde tüketildi.

        Some(Node::ControlFlow { keyword, value })
    }

    // struct StructName { var member: Type; ... }
    fn parse_struct_declaration(&mut self) -> Option<Node> {
        self.next_token(); // 'struct' tüket

        let name = if let Token::Identifier(name) = self.current_token.clone() {
            self.next_token();
            name
        } else {
            return None;
        };

        if self.current_token != Token::LBrace { return None; }
        self.next_token(); // '{' tüket

        let mut members = Vec::new();

        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            // Sadece değişken tanımlamaları (Declaration) bekleniyor.
            if self.current_token == Token::KeywordVar || self.current_token == Token::KeywordConst {
                let declaration_node = self.parse_declaration()?; 
                members.push(Box::new(declaration_node));
            } else {
                // Hata: Struct içinde sadece değişken tanımı (var/const) bekleniyor.
                return None; 
            }
        }

        if self.current_token != Token::RBrace { return None; }
        self.next_token(); // '}' tüket

        Some(Node::StructDeclaration { name, members })
    }

    // extend MyStruct { fn method() { ... } }
    fn parse_extend_block(&mut self) -> Option<Node> {
        self.next_token(); // 'extend' tüket

        let target_type = self.parse_type()?;

        if self.current_token != Token::LBrace { return None; }
        self.next_token(); // '{' tüket

        let mut methods = Vec::new();

        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            // Sadece fonksiyon tanımları (metotlar) bekleniyor.
            if self.current_token == Token::KeywordFn || self.current_token == Token::KeywordInline {
                methods.push(self.parse_function_declaration()?);
            } else {
                // Hata: Extend bloğu içinde sadece fonksiyon (metot) bekleniyor.
                return None;
            }
        }

        if self.current_token != Token::RBrace { return None; }
        self.next_token(); // '}' tüket

        Some(Node::ExtendBlock { target_type, methods })
    }

    // typedef NewName = OldType;
    fn parse_typedef_declaration(&mut self) -> Option<Node> {
        self.next_token(); // 'typedef' tüket

        let new_name = if let Token::Identifier(name) = self.current_token.clone() {
            self.next_token();
            name
        } else {
            return None;
        };

        if self.current_token != Token::Assign { return None; } // '=' bekleniyor
        self.next_token(); // '=' tüket

        let base_type = self.parse_type()?;

        if !self.expect_peek(&Token::Semicolon) { return None; }
        // self.next_token(); // ';' expect_peek içinde tüketildi.

        Some(Node::TypedefDeclaration { new_name, base_type })
    }
    
    // enum Color { RED, GREEN = 10, BLUE }
    fn parse_enum_declaration(&mut self) -> Option<Node> {
        self.next_token(); // 'enum' tüket

        let name = if let Token::Identifier(name) = self.current_token.clone() {
            self.next_token();
            name
        } else {
            return None;
        };

        if self.current_token != Token::LBrace { return None; }
        self.next_token(); // '{' tüket

        let mut members = Vec::new();

        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            let member_name = if let Token::Identifier(name) = self.current_token.clone() {
                self.next_token();
                name
            } else {
                return None;
            };
            
            let mut value = None;
            if self.current_token == Token::Assign { // = value
                self.next_token(); // '=' tüket
                
                // Sadece tamsayı literal beklenir
                if let Token::Int(val_str) = self.current_token.clone() {
                    self.next_token();
                    value = val_str.parse::<i64>().ok(); 
                } else {
                    return None;
                }
            }
            members.push((member_name, value));

            // Virgülden sonraki öğeyi bekle
            if self.current_token == Token::Comma {
                self.next_token(); // ',' tüketildi.
            } else if self.current_token != Token::RBrace {
                return None;
            }
        }

        if self.current_token != Token::RBrace { return None; }
        self.next_token(); // '}' tüket

        Some(Node::EnumDeclaration { name, members })
    }

    // match target { pattern -> result, _ -> default }
    fn parse_match_expression(&mut self) -> Option<Node> {
        self.next_token(); // 'match' tüket

        let target = self.parse_expression().map(Box::new)?;

        if self.current_token != Token::LBrace { return None; }
        self.next_token(); // '{' tüket

        let mut arms = Vec::new();

        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            // 1. Pattern/Koşul Ayrıştırma
            let pattern = self.parse_expression()?;
            
            if self.current_token != Token::Arrow { return None; } // '->' bekleniyor
            self.next_token(); // '->' tüket

            // 2. Result/Gövde Ayrıştırma
            let result = self.parse_expression()?;
            
            arms.push((pattern, result));

            // Virgül veya kapanış parantezi beklenir
            if self.current_token == Token::Comma {
                self.next_token(); // ',' tüketildi.
            } else if self.current_token != Token::RBrace {
                return None;
            }
        }

        if self.current_token != Token::RBrace { return None; }
        self.next_token(); // '}' tüket

        Some(Node::MatchExpression { target, arms })
    }

    // --- YARDIMCI İFADE AYRIŞTIRICILARI ---

    // Tekli (Prefix) ifadeyi ayrıştırır.
	fn parse_prefix_expression(&mut self) -> Option<Node> {
		let operator = self.current_token.clone();
		self.next_token(); // Operatörü atla
		
		// Öncelikli olarak ayrıştırılacaklar:
		match operator {
			// Düzeltildi: Star
			Token::Star | Token::Ampersand | Token::Bang | Token::Minus => self.parse_expression_with_precedence(Precedence::Prefix),
			_ => None,
		}
	}

    // struct.member veya group.label
    fn parse_member_access(&mut self, object: Node) -> Option<Node> {
        // Current token: Identifier (member)
        
        let member = match self.current_token.clone() {
            Token::Identifier(name) => {
                self.next_token();
                Node::Identifier(name)
            }
            _ => {
                return None;
            }
        };
        
        Some(Node::MemberAccess {
            object: Box::new(object),
            member: Box::new(member),
        })
    }

    // array[index]
    fn parse_index_expression(&mut self, array: Node) -> Option<Node> {
        // Current token: LBracket ( [ )
        // self.next_token(); // '[' expect_peek içinde tüketildi.

        let index_expression = self.parse_expression()?;

        if !self.expect_peek(&Token::RBracket) {
            return None;
        }

        Some(Node::MemberAccess { 
            object: Box::new(array),
            member: Box::new(index_expression),
        })
    }

    // Fonksiyon Çağrısı Ayrıştırma
    fn parse_call_expression(&mut self, function: Node) -> Option<Node> {
        // Current token: LParen ( ( )
        // self.next_token(); // '(' expect_peek içinde tüketildi.

        let mut arguments = Vec::new();

        while self.current_token != Token::RParen && self.current_token != Token::EOF {
            let mut arg_name: Option<String> = None;
            let mut arg_value: Option<Node> = None;

            // İsimli Argüman Kontrolü (name: value)
            if let Token::Identifier(name) = self.current_token.clone() {
                if self.peek_token == Token::Colon {
                    arg_name = Some(name);
                    self.next_token(); // İsim tüketildi
                    self.next_token(); // ':' tüketildi
                }
            }

            // Değeri Ayrıştır (İsimli veya Pozisyonel Argüman Değeri)
            arg_value = self.parse_expression();

            if let Some(value) = arg_value {
                arguments.push(Argument { name: arg_name, value });
            } else {
                return None;
            }
            
            // Virgülden sonraki argümanı bekle
            if self.current_token == Token::Comma {
                self.next_token(); // ',' tüketildi.
            } else if self.current_token != Token::RParen {
                return None;
            }
        }

        if !self.expect_peek(&Token::RParen) { return None; }
        // self.next_token(); // ')' expect_peek içinde tüketildi.

        Some(Node::CallExpression {
            function: Box::new(function),
            arguments,
        })
    }

    // Lambda/Anonim Fonksiyon Ayrıştırma
    fn parse_lambda_expression(&mut self) -> Option<Node> {
        // Current token: KeywordFn (fn)
        self.next_token(); // 'fn' tüketildi.
        
        // 1. Parametreler
        let params = self.parse_parameter_list()?;

        // 2. Dönüş Tipi Al (:i32)
        if !self.expect_peek(&Token::Colon) { return None; }
        // self.next_token(); // ':' expect_peek içinde tüketildi.
        
        let return_type = self.parse_type()?;

        // 3. Ok İşareti '->' bekleniyor
        if self.current_token != Token::Arrow {
            return None;
        }
        self.next_token(); // '->' tüketildi.

        // 4. İfadeyi Ayrıştır (Gövde tek bir ifadedir: x * 2)
        let body_expression = self.parse_expression()?;

        // Anonim fonksiyonu, gövdesi tek bir return olan bir FunctionDeclaration'a dönüştürüyoruz.
        let return_stmt = Node::ControlFlow { 
            keyword: Token::KeywordReturn, 
            value: Some(Box::new(body_expression))
        };
        let body = Node::BlockStatement(vec![return_stmt]);

        Some(Node::FunctionDeclaration {
            name: "".to_string(), // Anonim isim
            params,
            return_type,
            is_inline: false,
            body: Box::new(body),
        })
    }

    // --- YARDIMCI TIP AYRIŞTIRICILARI ---

    // [ & | * ] Identifier [ [ Expression ] ]
    fn parse_type(&mut self) -> Option<Type> {
        let mut is_ref = false;
        let mut is_ptr = false;

        // Referans veya İşaretçi öneki
        if self.current_token == Token::Ampersand { is_ref = true; self.next_token(); }
        else if self.current_token == Token::Star { is_ptr = true; self.next_token(); }

        // Tip ismini al (i32, f32, CustomStruct, vs.)
        let base_type_name = match self.current_token.clone() {
            Token::Identifier(name) => { self.next_token(); name },
            // Temel tipler de Identifier olarak ele alınabilir (i32, f32, str, vs.)
            _ => { return None; } 
        };
        
        let mut final_type = Type::Identifier(base_type_name);
        
        // Array tipi kontrolü: []
        if self.current_token == Token::LBracket {
            self.next_token(); // '[' tüket
            let size = if self.current_token != Token::RBracket {
                self.parse_expression().map(Box::new) // Array boyutu (örn: [4])
            } else { None };
            
            if !self.expect_peek(&Token::RBracket) { return None; } // ']' bekleniyor
            // self.next_token(); // ']' expect_peek içinde tüketildi.
            
            final_type = Type::Array(Box::new(final_type), size);
        }

        if is_ref { Some(Type::Reference(Box::new(final_type))) }
        else if is_ptr { Some(Type::Pointer(Box::new(final_type))) }
        else { Some(final_type) }
    }

	// Literal İfadeyi Ayrıştırır (Sayı, String, Boolean, Null)
    fn parse_literal_expression(&mut self) -> Option<Node> {
        let token = self.current_token.clone();
        match token {
            // E0599 Fix: IntegerLiteral/FloatLiteral/StringLiteral/BooleanLiteral/NullLiteral eklendi
            Token::Int(value) => Some(Node::IntegerLiteral(value)),
            Token::Float(value) => Some(Node::FloatLiteral(value)),
            Token::String(value) => Some(Node::StringLiteral(value)),
            Token::True => Some(Node::BooleanLiteral(true)),
            Token::False => Some(Node::BooleanLiteral(false)),
            // E0599 Fix: Token::Null yerine Token::KeywordNull kullanıldı (Lexer çıktısıyla tutarlılık için)
            Token::KeywordNull => Some(Node::NullLiteral),
            _ => {
                self.add_error(format!("Beklenmeyen literal token: {:?}", token)); // E0599 Fix: add_error
                None
            }
        }
    }
	// Tanımlayıcıyı (Identifier) Ayrıştırır
    fn parse_identifier_expression(&mut self) -> Option<Node> {
        if let Token::Identifier(name) = self.current_token.clone() {
            Some(Node::Identifier(name))
        } else {
            self.add_error(String::from("Beklenmeyen token. Tanımlayıcı bekleniyordu."));
            None
        }
    }
    
    // Tekli Operatörler (Prefix)
    fn parse_prefix_expression(&mut self) -> Option<Node> {
        let operator = self.current_token.clone();
        self.next_token(); // Operatörü tüket

        // Önceliği Prefix'ten daha yüksek olan ifadeleri ayrıştırır (Örn: -a * b)
		let right = self.parse_expression_with_precedence(Precedence::Prefix)?; 

		Some(Node::UnaryExpression {
			operator,
			right: Box::new(right),
		})
	}
	
    // Parametre Listesi Ayrıştırma
    fn parse_parameter_list(&mut self) -> Option<Vec<Parameter>> {
        let mut params = Vec::new();

        if !self.expect_peek(&Token::LParen) { return None; } // '(' bekleniyor
        // self.next_token(); // '(' expect_peek içinde tüketildi.

        while self.current_token != Token::RParen && self.current_token != Token::EOF {
            // Parametre Adı
            let name = if let Token::Identifier(name) = self.current_token.clone() {
                self.next_token();
                name
            } else {
                return None;
            };
            
            // Tip Ayracı ':' bekleniyor
            if !self.expect_peek(&Token::Colon) { return None; }
            // self.next_token(); // ':' expect_peek içinde tüketildi.

            // Tip Al
            let type_hint = self.parse_type()?;
            
            let mut default_value = None;
            // Opsiyonel Argüman Kontrolü ( = değer )
            if self.current_token == Token::Assign {
                self.next_token(); // '=' tüketildi.
                default_value = self.parse_expression(); // İfadeyi (varsayılan değeri) ayrıştırır.
            }

            // Box<Node> olarak sarıldı
            params.push(Parameter { name, type_hint, default_value: default_value.map(Box::new) });

            // Virgülden sonraki parametreyi bekle
            if self.current_token == Token::Comma {
                self.next_token(); // ',' tüketildi.
            } else if self.current_token != Token::RParen {
                return None;
            }
        }

        if !self.expect_peek(&Token::RParen) { return None; } // ')' bekleniyor
        // self.next_token(); // ')' expect_peek içinde tüketildi.

        Some(params)
    }

    // --- MODÜL/GROUP AYRIŞTIRICILARI ---

    // :modül_adı;
    fn parse_module_prefix(&mut self) -> Option<Node> {
        // Current token: Colon (:)
        self.next_token(); // ':' tüketildi.

        if let Token::Identifier(name) = self.current_token.clone() {
            self.next_token(); // Modül ismini tüket

            if !self.expect_peek(&Token::Semicolon) {
                return None;
            }

            return Some(Node::ModulePrefix(name));
        } else {
            return None;
        }
    }
    
    // use math { Add, PI };
    fn parse_use_statement(&mut self) -> Option<Node> {
        self.next_token(); // 'use' tüketildi.

        let module_name = if let Token::Identifier(name) = self.current_token.clone() {
            self.next_token();
            name
        } else {
            return None;
        };

        let mut imported_items = Vec::new();

        if self.current_token == Token::LBrace { // Seçici İçe Aktarma
            self.next_token(); // '{' tüketildi.

            while self.current_token != Token::RBrace && self.current_token != Token::EOF {
                if let Token::Identifier(item_name) = self.current_token.clone() {
                    imported_items.push(item_name);
                    self.next_token();
                } else {
                    return None;
                }

                if self.current_token == Token::Comma {
                    self.next_token();
                } else if self.current_token != Token::RBrace {
                    return None;
                }
            }
            
            if !self.expect_peek(&Token::RBrace) {
                 return None;
            }
        }
        
        if !self.expect_peek(&Token::Semicolon) {
            return None;
        }

        Some(Node::UseStatement {
            module_name,
            imported_items
        })
    }
    
    // export const PI = 3.14;
    fn parse_export_statement(&mut self) -> Option<Node> {
        self.next_token(); // 'export' tüketildi.

        let exported_node = match self.current_token {
            Token::KeywordConst | Token::KeywordVar => self.parse_declaration(),
            Token::KeywordFn => self.parse_function_declaration(),
            Token::KeywordGroup => self.parse_group_declaration(),
            Token::KeywordStruct => self.parse_struct_declaration(),
            _ => {
                return None;
            }
        };

        exported_node.map(|node| Node::ExportStatement(Box::new(node)))
    }
        
    // group MyGroup(x:i32) { ... }
    fn parse_group_declaration(&mut self) -> Option<Node> {
        self.next_token(); // 'group' tüketildi.

        let name = if let Token::Identifier(name) = &self.current_token {
            self.next_token();
            name.clone()
        } else {
            return None;
        };

        let params = self.parse_parameter_list()?;

        if self.current_token != Token::LBrace {
            return None;
        }
        self.next_token(); // '{' tüketildi.

        let mut body = Vec::new();

        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            if let Some(member_node) = self.parse_statement() { // Group içindeki deyimleri ayrıştırır.
                body.push(member_node);
            } else {
                // Hata durumunda ilerle
                self.next_token();
            }
        }

        if self.current_token != Token::RBrace {
            return None;
        }
        self.next_token(); // '}' tüketildi.

        Some(Node::GroupDeclaration {
            name,
            params,
            body,
        })
    }

    // fn add(a:f32):f32 { ... }
    fn parse_function_declaration(&mut self) -> Option<Node> {
        let mut is_inline = false;
        
        if self.current_token == Token::KeywordInline {
            is_inline = true;
            self.next_token();
        }

        self.next_token(); // 'fn' tüketildi.
        
        let name = if let Token::Identifier(name) = self.current_token.clone() {
            self.next_token();
            name
        } else {
            return None;
        };

        let params = self.parse_parameter_list()?;

        if !self.expect_peek(&Token::Colon) { return None; }
        // self.next_token(); // ':' expect_peek içinde tüketildi.
        
        let return_type = self.parse_type()?;

        let body = if self.current_token == Token::LBrace {
            self.parse_block_statement()?
        } else {
            return None;
        };
        
        Some(Node::FunctionDeclaration {
            name,
            params,
            return_type,
            is_inline,
            body: Box::new(*body),
        })
    }

    // { statement1; statement2; }
    fn parse_block_statement(&mut self) -> Option<Box<Node>> {
        self.next_token(); // '{' tüket
        let mut statements = Vec::new();

        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                // Hata durumunda kurtarma (Basit)
                self.next_token(); 
            }
        }
        
        if self.current_token == Token::RBrace {
            self.next_token(); // '}' tüket
            Some(Box::new(Node::BlockStatement(statements)))
        } else {
            return None;
        }
    }

    // rolling: ETİKET;
    fn parse_rolling_statement(&mut self) -> Option<Node> {
        self.next_token(); // 'rolling' tüketildi.

        if self.current_token != Token::Colon { return None; }
        self.next_token(); // ':' tüketildi.

        let label = if let Token::Identifier(name) = self.current_token.clone() {
            self.next_token();
            name
        } else {
            return None;
        };

        if !self.expect_peek(&Token::Semicolon) {
            return None;
        }

        Some(Node::RollingStatement { label })
    }

    // fast_exec: TEST { ... }
    fn parse_fast_exec_block(&mut self) -> Option<Node> {
        self.next_token(); // "fast_exec" tüketildi.

        if self.current_token != Token::Colon { return None; }
        self.next_token(); // ':' tüketildi.

        let label = if let Token::Identifier(name) = self.current_token.clone() {
            self.next_token();
            name
        } else {
            return None;
        };

        if self.current_token != Token::LBrace { return None; }

        let body_node = self.parse_block_statement()?;

        Some(Node::LabeledBlock {
            label,
            body: Box::new(*body_node),
            is_fast_exec: true,
        })
    }

    // REQUEST_BLOCK: { ... } (Genel etiketli blok)
    fn parse_labeled_block(&mut self, is_fast_exec: bool) -> Option<Node> {
        let label = if let Token::Identifier(name) = self.current_token.clone() {
            self.next_token();
            name
        } else {
            return None;
        };

        if self.current_token != Token::Colon { return None; }
        self.next_token(); // ':' tüketildi.

        if self.current_token != Token::LBrace { return None; }

        let body_node = self.parse_block_statement()?;

        Some(Node::LabeledBlock {
            label,
            body: Box::new(*body_node),
            is_fast_exec,
        })
    }
}