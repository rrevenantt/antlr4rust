grammar Perf;

stat : expr ';'
     | expr '.'
     ;

expr
	: ID
	| 'not' expr
	| expr 'and' expr
    | expr 'or' expr
    | '(' ID ')' expr
    | expr '?' expr ':' expr
    | 'between' expr 'and' expr
	;

ID: [a-zA-Z_][a-zA-Z_0-9]*;
WS: [ \t\n\r\f]+ -> skip;