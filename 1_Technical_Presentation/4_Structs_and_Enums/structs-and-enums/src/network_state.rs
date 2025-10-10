use crate::Node;

#[derive(Default, Debug, Clone)]
pub struct NetworkState {
    pub validator_id: u64,
    pub validator_name: String,
    pub node: Node,
}
impl NetworkState {
    fn static_function() {
        println!("static");
    }

    pub fn display(&self) {
        NetworkState::static_function();
        println!(
            "network state: {} {}",
            self.validator_id, self.validator_name
        );
    }
    pub fn modify(&mut self) {
        self.validator_name = "other".to_string();
    }
}
