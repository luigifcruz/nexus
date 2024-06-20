// This function creates a serial number for the replicant.
// The serial number is a string that uniquely identifies the replicant.
// Example:
//    N6MAA10816
//    ^^ N6 is the model number (Nexus 6)
//      ^ M/F is randomly assigned
//       ^^ A/B/C two letter combination is randomly assigned
//         ^^^^^ five digit number is randomly assigned

use std::fmt;

pub struct SerialNumber {
    model: String,
}

impl SerialNumber {
    pub fn new() -> SerialNumber {
        SerialNumber {
            model: Self::generate(),
        }
    }

    fn generate() -> String {
        let m0 = "N6";
        let s0 = if rand::random() { "M" } else { "F" };
        let a1 = ['A', 'B', 'C'][rand::random::<usize>() % 3].to_string();
        let a2 = ['A', 'B', 'C'][rand::random::<usize>() % 3].to_string();
        let d1 = format!("{:05}", rand::random::<u32>() % 100000);
        return format!("{}{}{}{}{}", m0, s0, a1, a2, d1).to_string();
    }
}

impl fmt::Display for SerialNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.model)
    }
}