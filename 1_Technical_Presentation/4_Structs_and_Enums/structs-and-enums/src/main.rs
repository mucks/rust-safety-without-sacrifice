use crate::network_state::NetworkState;

mod network_state;

impl NetworkState {}

#[derive(Default, Debug, Clone)]
struct Node {}

fn print_state(ns: NetworkState) {
    println!("network debug: {ns:?}");
}

enum NetworkStates {
    ForestCity(NetworkState),
    Zuitzerland(String),
    Prospera(u32),
}

impl NetworkStates {
    fn display(&self) {}
}

fn main() {
    // Structs

    let mut new_state = NetworkState {
        validator_id: 1,
        validator_name: "Max".to_string(),
        node: Node {},
    };
    new_state.modify();
    let mut other_state = new_state.clone();
    let NetworkState {
        mut validator_id, ..
    } = other_state;
    validator_id = 2;
    new_state.display();

    let default_state = NetworkState::default();
    println!("default_state: {default_state:?}");

    // Enums

    let ns = NetworkStates::Zuitzerland("hello world".to_string());

    match ns {
        NetworkStates::ForestCity(ns) => println!("{ns:?}"),
        NetworkStates::Zuitzerland(s) => println!("{s}"),
        NetworkStates::Prospera(u) => println!("{u}"),
    }
}
