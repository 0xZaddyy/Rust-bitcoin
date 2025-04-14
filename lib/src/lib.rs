use uint::construct_uint;
pub mod crypto;
pub mod sha256;
pub mod types;
pub mod util;

construct_uint! {
    #[derive(Serialize, Deserialize)]
    pub struct U256(4);
}
