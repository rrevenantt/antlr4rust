grammar CSV;

@tokenfactory{
pub type LocalTokenFactory<'input> = antlr_rust::common_token_factory::ArenaCommonFactory<'input>;
}

csvFile: hdr row+ ;
hdr : row ;

row : field (',' field)* '\r'? '\n' {println!("test");};

field
    : TEXT
    | STRING 
    |
    ;

WS     : [ ]+ -> channel(HIDDEN);
TEXT   : ~[ ,\n\r"]+ ;
STRING : '"' ('""'|~'"')* '"' ; // quote-quote is an escaped quote
