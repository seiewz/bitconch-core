use pubkey::Pubkey;

/// An Account with userdata that is stored on chain
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// define a publicstruct of Account
pub struct Account {
    /// tokens in the account
    pub tokens: i64,
    /// user data
    /// A transaction can write to its userdata
    pub userdata: Vec<u8>,
    /// contract id this contract belongs to
    pub program_id: Pubkey,
}

/// defining an new method on the Account struct, return a instance of Account
impl Account {
    pub fn new(tokens: i64, space: usize, program_id: Pubkey) -> Account {
        Account {
            tokens,
            userdata: vec![0u8; space],
            program_id,
        }
    }
}

#[derive(Debug)]
/// define a publicstruct of KeyedAccount
pub struct KeyedAccount<'a> {
    pub key: &'a Pubkey,
    pub account: &'a mut Account,
}
