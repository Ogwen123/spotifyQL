pub enum Action {
    RunQuery(String),
    Internal // For when the does not need to emit an action
}