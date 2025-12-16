use crate::query::statements::{Aggregation, SelectStatement};
use crate::query::tokenise::{Attribute, DataSource, Operator, Token};

pub fn parse(tokens: Vec<Token>) -> SelectStatement {
    // if the tokens contain a COUNT then it's a SelectCount, otherwise it's a Select

    let mut aggregation = Aggregation::None;
    let targets: Vec<Attribute> = Vec::new();

    SelectStatement {
        aggregation: Aggregation::None,
        targets: vec![Attribute::Id],
        source: DataSource::Playlist(String::new()),
        condition: Some((Attribute::Id, Operator::Equals, String::new())),
    }
}
