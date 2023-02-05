pub struct Signal {
    pub name: String,
    pub edge_direction: Option<EdgeDirection>,
    pub states: Vec<SignalState>
}

pub enum EdgeDirection {
    Positive,
    Negative
}

pub enum SignalState {
    High,
    Low,
    Value(String)
}