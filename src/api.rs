use crate::utils::url::build_url;

struct APIQuery {
    query: String,
    types: Vec<QueryType>,
}

pub enum QueryType {
    Artist,
    Playlist,
    Track,
    Show,
    Episode,
    Audiobook,
    None,
}

type QueryResult = ();
impl<'a> APIQuery {
    const ENDPOINT: &'a str = "https://api.spotify.com/v1/search";

    pub fn new() {}

    pub fn query<T: ToString>(mut self, q: T) -> Self {
        self.query = q.to_string();
        self
    }

    pub fn query_type(mut self, _type: QueryType) -> Self {
        self.types.push(_type);
        self
    }

    /// Consumes the struct and sends the built request
    pub fn send(self) -> Result<QueryResult, String> {
        let url = Self::ENDPOINT.to_string();

        let mut params: Vec<(&str, String)> = Vec::new();
        // build the param list
        params.push(("q", self.query));

        let final_url = build_url(url, params);

        Ok(())
    }
}
