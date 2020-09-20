#![feature(try_blocks)]
#![feature(inner_deref)]
#![feature(test)]
#[macro_use]
extern crate lazy_static;
extern crate test;

mod gen {
    use test::Bencher;

    use antlr_rust::common_token_stream::CommonTokenStream;
    use antlr_rust::InputStream;

    // use crate::gen::perflexer::PerfLexer;
    // use crate::gen::perfparser::PerfParser;
    // mod perflexer;
    // mod perfparser;
    // mod perflistener;

    // #[cfg(not(debug_assertions))]
    // #[test]
    // fn performance(){
    #[ignore]
    // #[bench]
    fn performance(b: &mut Bencher) {
        // b.iter(|| {
        //     let lexer = PerfLexer::new(Box::new(InputStream::new(input.to_string())));
        //     let source = CommonTokenStream::new(lexer);
        //     let mut parser = PerfParser::new(Box::new(source));
        //     let result = parser.stat().expect("oops");
        // });
    }

    const input: &str = "\
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4 and
		 between X1 and X2 or between X3 and X4
		 ;";
}
