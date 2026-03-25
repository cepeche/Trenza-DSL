use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effect {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DispatchResult {
    pub new_state: String,
    pub concurrent_states: Vec<String>,
    pub triggered_effects: Vec<Effect>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemState {
    pub current_state: String,
    pub concurrent_states: Vec<String>,
}
