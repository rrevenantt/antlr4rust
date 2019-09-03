#![crate_type = "lib"]
#![feature(underscore_lifetimes)]
#![feature(try_blocks)]
#![feature(nll)]
#![feature(raw)]
#![feature(inner_deref)]
#![feature(is_sorted)]
#[macro_use]
extern crate lazy_static;
extern crate byteorder;
extern crate uuid;

//pub mod ll1_analyzer;
pub mod common_token_factory;
pub mod recognizer;
pub mod int_stream;
pub mod lexer_action;
pub mod atn_simulator;
pub mod atn_config;
//pub mod tokenstream_rewriter;
pub mod semantic_context;
pub mod dfa_state;
pub mod atn_state;
//pub mod parser_rule_context;
pub mod prediction_context;
pub mod interval_set;
pub mod token_source;
pub mod atn_deserialization_options;
pub mod token_stream;
pub mod char_stream;
//pub mod trace_listener;
pub mod transition;
//pub mod tree;
pub mod dfa;
//pub mod file_stream;
pub mod atn_deserializer;
pub mod token;
//pub mod utils;
//pub mod trees;
pub mod atn_config_set;
//pub mod diagnostic_error_listener;
pub mod error_listener;
//pub mod prediction_mode;
pub mod input_stream;
//pub mod common_token_stream;
pub mod lexer;
//pub mod dfa_serializer;
pub mod lexer_atn_simulator;
pub mod atn;
pub mod errors;
pub mod error_strategy;
//pub mod lexer_action_executor;
pub mod parser;
pub mod parser_atn_simulator;
//pub mod tokenstream_rewriter_test;
pub mod atn_type;
pub mod rule_context;

//
mod test {
    mod csv_lexer;
    //    mod CSVLexer;
    mod csv_parser;

    use csv_lexer::*;
    use crate::input_stream::InputStream;
    use std::io::{stdout, Write};
    use crate::token_source::TokenSource;
    use crate::recognizer::Recognizer;
    use crate::token_stream::UnbufferedTokenStream;
    use crate::test::csv_parser::{CSVParser, csvFile};

    #[test]
    fn lexer_test() {
        println!("test started");
        let mut _lexer = CSVLexer::new(Box::new(InputStream::new("V123,V2\nd1,d2".into())));
        //        _lexer.base.add_error_listener();
        let mut token_source = UnbufferedTokenStream::new(_lexer);
        let mut token_source_iter = token_source.iter();
        assert_eq!(token_source_iter.next().unwrap(), 4);
        assert_eq!(token_source_iter.next().unwrap(), 1);
        assert_eq!(token_source_iter.next().unwrap(), 4);
        assert_eq!(token_source_iter.next().unwrap(), 3);
        assert_eq!(token_source_iter.next().unwrap(), 4);
        assert_eq!(token_source_iter.next().unwrap(), 1);
        assert_eq!(token_source_iter.next().unwrap(), 4);
        assert_eq!(token_source_iter.next(), None);

//        println!("Token {:?}",&token);
//        stdout().flush();
//        panic!("delete when finish");
    }

    #[test]
    fn parser_test() {
        println!("test started");
        let mut _lexer = CSVLexer::new(Box::new(InputStream::new("V123,V2\nd1,d2\n".into())));
        //        _lexer.base.add_error_listener();
        let mut token_source = UnbufferedTokenStream::new(_lexer);
        let mut parser = CSVParser::new(Box::new(token_source));
        println!("\nstart parsing");
        let result = parser.csvFile();
        assert!(result.is_ok())
    }
}
