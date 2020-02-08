pub mod index;
pub mod tokenizer;

fn main() {
    let mut ii = index::InvertedIndex::new();
    //println!("{:?}",&ii);
    ii.import_document(1, "Hallo hallo hallo test");
    //println!("first import done {:?}", ii.clone());
    ii.import_document(2, "Hallo, Hallo hsdshah hallo");
    ii.import_document(3, "keine ahnung hsdshah");
    ii.query_processing("keine ahnung");
}