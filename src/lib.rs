#![crate_type = "lib"]
#![feature(underscore_lifetimes)]
#![feature(try_blocks)]
#![feature(nll)]
#![feature(raw)]
#![feature(inner_deref)]
#![feature(is_sorted)]
#![feature(bind_by_move_pattern_guards)]
#![feature(never_type)]
#![feature(cell_update)]
#[macro_use]
extern crate lazy_static;
extern crate byteorder;
//extern crate uuid;

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
pub mod dfa_serializer;
pub mod lexer_atn_simulator;
pub mod atn;
pub mod errors;
pub mod error_strategy;
pub mod lexer_action_executor;
pub mod parser;
pub mod parser_atn_simulator;
//pub mod tokenstream_rewriter_test;
pub mod atn_type;
pub mod rule_context;

//
mod test {
    //    mod csv_lexer;
//    mod csv_parser;
    mod xml_lexer;

    //    use csv_lexer::*;
//    use crate::test::csv_parser::{CSVParser, csvFile};
    use crate::input_stream::InputStream;
    use crate::token_stream::{UnbufferedTokenStream, TokenStream};
    use std::io::{stdout, Write};
    use crate::token_source::TokenSource;
    use crate::recognizer::Recognizer;
    use crate::test::xml_lexer::XMLLexer;
    use crate::int_stream::IntStream;
    use crate::token::TOKEN_EOF;
    use std::iter::FromIterator;
    use std::ops::Deref;
//    <?do not care?>
//    <Container>

    #[test]
    fn lexer_test_xml() -> std::io::Result<()> {
//        println!("test started");
//        let input = std::fs::read_to_string(std::env::current_dir()?.join("input"))?;
        let data =
            r#"<?xml version="1.0"?>
<!--comment-->>
<?xml-stylesheet type="text/css" href="nutrition.css"?>
<script>
<![CDATA[
function f(x) {
if (x < x && a > 0) then duh
}
]]>
</script>"#.to_owned();
        let mut _lexer = XMLLexer::new(Box::new(InputStream::new(data.clone())));
        //        _lexer.base.add_error_listener();
        let a = "a".to_owned() + "";
        let mut string = String::new();
        {
            let mut token_source = UnbufferedTokenStream::new(&mut _lexer);
            while token_source.la(1) != TOKEN_EOF {
                {
                    let token = token_source.lt(1);

                    let len = token.get_stop() as usize + 1 - token.get_start() as usize;
                    string.extend(
                        format!("{},len {}:\n{}\n",
                                xml_lexer::_SYMBOLIC_NAMES[token.get_token_type() as usize].unwrap_or(&format!("{}", token.get_token_type())),
                                len,
                                String::from_iter(data.chars().skip(token.get_start() as usize).take(len))
                        ).chars());
//                println!("result {}",token.get_token_type())
                }
                token_source.consume();
            }
        }
        println!("{}", string);
        println!("{}", _lexer.get_interpreter().unwrap().get_dfa().to_lexer_String());
//        println!("{:?}",token_source_iter.map(|x|xml_lexer::ruleNames[x as usize]).collect::<Vec<&str>>());
        panic!()
    }
//
//    #[test]
//    fn lexer_test_csv() {
//        println!("test started");
//        let mut _lexer = CSVLexer::new(Box::new(InputStream::new("V123,V2\nd1,d2".into())));
//        //        _lexer.base.add_error_listener();
//        let mut token_source = UnbufferedTokenStream::new(_lexer);
//        let mut token_source_iter = token_source.iter();
//        assert_eq!(token_source_iter.next().unwrap(), 4);
//        assert_eq!(token_source_iter.next().unwrap(), 1);
//        assert_eq!(token_source_iter.next().unwrap(), 4);
//        assert_eq!(token_source_iter.next().unwrap(), 3);
//        assert_eq!(token_source_iter.next().unwrap(), 4);
//        assert_eq!(token_source_iter.next().unwrap(), 1);
//        assert_eq!(token_source_iter.next().unwrap(), 4);
//        assert_eq!(token_source_iter.next(), None);
//
////        println!("Token {:?}",&token);
////        stdout().flush();
////        panic!("delete when finish");
//    }
//
//    #[test]
//    fn parser_test_csv() {
//        println!("test started");
//        let mut _lexer = CSVLexer::new(Box::new(InputStream::new("V123,V2\nd1,d2\n".into())));
//        //        _lexer.base.add_error_listener();
//        let mut token_source = UnbufferedTokenStream::new(_lexer);
//        let mut parser = CSVParser::new(Box::new(token_source));
//        println!("\nstart parsing");
//        let result = parser.csvFile();
//        assert!(result.is_ok())
//    }
}
