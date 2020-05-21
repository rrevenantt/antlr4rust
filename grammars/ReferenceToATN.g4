grammar ReferenceToATN;

@tokenfactory{
pub type LocalTokenFactory<\'input> = antlr_rust::token_factory::OwningTokenFactory;
}

a : (ID|ATN)* ATN? {println!("{}",$text);};
ID : 'a'..'z'+ ;
ATN : '0'..'9'+;
WS : (' '|'\n') -> skip ;
