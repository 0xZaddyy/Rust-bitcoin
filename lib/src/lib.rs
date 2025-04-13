use uint::construct_uint;



pub mod sha256;
pub mod types;
pub mod util;
pub mod crypto;


construct_uint!{
    #[derive(Serialize, Deserialize)]
    pub struct U256(4);
}