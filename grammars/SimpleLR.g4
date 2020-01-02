grammar SimpleLR;
s @after {println!("test");} : a ;
a : a ID
  | ID
  ;
ID : 'a'..'z'+ ;
WS : (' '|'\n') -> skip ;

