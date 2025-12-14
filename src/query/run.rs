use crate::query::data::build_queries;
use crate::query::tokenise::tokenise;

pub fn run_query(query: String) -> Result<(), String> {
    let tokens = tokenise(query).map_err(|x| x)?;
    
    let api_queries = build_queries(tokens);
    
    for q in api_queries {
        println!("{:?}", q)
    }
    
    Ok(())
}