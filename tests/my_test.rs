#![feature(try_blocks)]
#![feature(inner_deref)]

extern crate antlr_rust;
#[macro_use]
extern crate lazy_static;

//fn main() {
//    println!("Hello, world!");
//}
mod gen {
    use std::io::{stdout, Write};
    use std::iter::FromIterator;
    use std::ops::{Deref, DerefMut};

    use antlr_rust::common_token_stream::CommonTokenStream;
    use antlr_rust::input_stream::InputStream;
    use antlr_rust::int_stream::IntStream;
    use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext};
    use antlr_rust::recognizer::Recognizer;
    use antlr_rust::token::{Token, TOKEN_EOF};
    use antlr_rust::token_source::TokenSource;
    use antlr_rust::token_stream::{TokenStream, UnbufferedTokenStream};
    use antlr_rust::tree::{ParseTreeListener, TerminalNodeCtx};
    use csvlexer::*;
    use csvlistener::*;
    use csvparser::CSVParser;
    use referencetoatnlexer::ReferenceToATNLexer;
    use referencetoatnlistener::ReferenceToATNListener;
    use referencetoatnparser::ReferenceToATNParser;
    use xmllexer::XMLLexer;

    use crate::gen::simplelrlexer::SimpleLRLexer;
    use crate::gen::simplelrlistener::SimpleLRListener;
    use crate::gen::simplelrparser::SimpleLRParser;

    mod csvlexer;
    mod csvparser;
    mod csvlistener;
    mod xmllexer;
    mod referencetoatnparser;
    mod referencetoatnlexer;
    mod referencetoatnlistener;
    mod simplelrparser;
    mod simplelrlexer;
    mod simplelrlistener;

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
            let mut token_source = UnbufferedTokenStream::new_unbuffered(&mut _lexer);
            while token_source.la(1) != TOKEN_EOF {
                {
                    let token = token_source.lt(1).unwrap();

                    let len = token.get_stop() as usize + 1 - token.get_start() as usize;
                    string.extend(
                        format!("{},len {}:\n{}\n",
                                xmllexer::_SYMBOLIC_NAMES[token.get_token_type() as usize].unwrap_or(&format!("{}", token.get_token_type())),
                                len,
                                String::from_iter(data.chars().skip(token.get_start() as usize).take(len))
                        ).chars());
//                println!("result {}",token.get_token_type())
                }
                token_source.consume();
            }
        }
        println!("{}", string);
        println!("{}", _lexer.get_interpreter().unwrap().get_dfa().to_lexer_string());
//        println!("{:?}",token_source_iter.map(|x|xml_lexer::ruleNames[x as usize]).collect::<Vec<&str>>());
//        panic!()
        Ok(())
    }

    //
    #[test]
    fn lexer_test_csv() {
        println!("test started lexer_test_csv");
        let mut _lexer = CSVLexer::new(Box::new(InputStream::new("V123,V2\nd1,d2".into())));
        //        _lexer.base.add_error_listener();
//        let mut token_source = UnbufferedTokenStream::new_unbuffered(_lexer);
        let mut token_source = CommonTokenStream::new(_lexer);
        let mut token_source_iter = token_source.iter();
        assert_eq!(token_source_iter.next().unwrap(), 5);
        assert_eq!(token_source_iter.next().unwrap(), 1);
        assert_eq!(token_source_iter.next().unwrap(), 5);
        assert_eq!(token_source_iter.next().unwrap(), 3);
        assert_eq!(token_source_iter.next().unwrap(), 5);
        assert_eq!(token_source_iter.next().unwrap(), 1);
        assert_eq!(token_source_iter.next().unwrap(), 5);
        assert_eq!(token_source_iter.next(), None);

//        println!("Token {:?}",&token);
//        stdout().flush();
//        panic!("delete when finish");
    }

    struct Listener {}

    impl ParseTreeListener for Listener {
        fn enter_every_rule(&self, ctx: &dyn ParserRuleContext) {
            println!("rule entered {}", csvparser::ruleNames.get(ctx.get_rule_index()).unwrap_or(&"error"))
        }
    }

    impl CSVListener for Listener {}

    #[test]
    fn parser_test_csv() {
        println!("test started");
        let mut _lexer = CSVLexer::new(Box::new(InputStream::new("V123, V2\nd1,d2\n".into())));
        //        _lexer.base.add_error_listener();
//        let mut token_source = UnbufferedTokenStream::new_unbuffered(_lexer);
        let mut token_source = CommonTokenStream::new(_lexer);
        let mut parser = CSVParser::new(Box::new(token_source));
        parser.add_parse_listener(Box::new(Listener {}));
        println!("\nstart parsing parser_test_csv");
        let result = parser.csvFile();
        assert!(result.is_ok())
    }

    struct Listener2 {}

    impl ParseTreeListener for Listener2 {
        fn enter_every_rule(&self, ctx: &dyn ParserRuleContext) {
            println!("rule entered {}", referencetoatnparser::ruleNames.get(ctx.get_rule_index()).unwrap_or(&"error"))
        }
    }

    impl ReferenceToATNListener for Listener2 {}

    #[test]
    fn adaptive_predict_test() {
        let mut _lexer = ReferenceToATNLexer::new(Box::new(InputStream::new("a 34 b".into())));
        //        _lexer.base.add_error_listener();
//        let mut token_source = UnbufferedTokenStream::new_unbuffered(_lexer);
        let mut token_source = CommonTokenStream::new(_lexer);
        let mut parser = ReferenceToATNParser::new(Box::new(token_source));
        parser.add_parse_listener(Box::new(Listener2 {}));
        println!("\nstart parsing adaptive_predict_test");
        let result = parser.a();
        assert!(result.is_ok())
    }

    struct Listener3;

    impl ParseTreeListener for Listener3 {
        fn visit_terminal(&self, node: &BaseParserRuleContext<TerminalNodeCtx>) {
            println!("terminal node {}", node.symbol.get_text());
        }

        fn enter_every_rule(&self, ctx: &dyn ParserRuleContext) {
            println!("rule entered {}", simplelrparser::ruleNames.get(ctx.get_rule_index()).unwrap_or(&"error"))
        }

        fn exit_every_rule(&self, ctx: &dyn ParserRuleContext) {
            println!("rule exited {}", simplelrparser::ruleNames.get(ctx.get_rule_index()).unwrap_or(&"error"))
        }
    }

    impl SimpleLRListener for Listener3 {}

    #[test]
    fn lr_test() {
        let mut _lexer = SimpleLRLexer::new(Box::new(InputStream::new("x y z".into())));
        //        _lexer.base.add_error_listener();
//        let mut token_source = UnbufferedTokenStream::new_unbuffered(_lexer);
        let mut token_source = CommonTokenStream::new(_lexer);
        let mut parser = SimpleLRParser::new(Box::new(token_source));
        parser.add_parse_listener(Box::new(Listener3));
        println!("\nstart parsing lr_test");
        let result = parser.s().expect("expected to parse successfully");
        println!("{}", result.to_string_tree(&*parser));
//        assert!(result.is_ok())
    }
}