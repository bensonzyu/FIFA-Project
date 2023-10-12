use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Player {
    pub overall: u8,
    pub value_eur: u32,
}
