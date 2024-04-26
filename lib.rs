#![cfg_attr(not(feature = "std"), no_std, no_main)]

use pop_api::nfts::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {
	InvalidCollection,
	NftsError(Error),
}

impl From<Error> for ContractError {
	fn from(value: Error) -> Self {
		ContractError::NftsError(value)
	}
}

#[ink::contract]
mod my_nfts {
    use super::*;

	#[ink(storage)]
	#[derive(Default)]
	pub struct MyNfts;

    impl MyNfts {
		#[ink(constructor, payable)]
		pub fn new() -> Self {
			ink::env::debug_println!("Nfts::new");
			Default::default()
		}

		#[ink(message)]
		pub fn create_nft_collection( &self ) -> Result<(), ContractError>{
			ink::env::debug_println!("Nfts::create_nft_collection: collection creation started.");
            let admin = Self::env().caller();
            let item_settings = ItemSettings(BitFlags::from(ItemSetting::Transferable));

            let mint_settings = MintSettings {
                mint_type: MintType::Issuer,
                price: Some(0),
                start_block: Some(0),
                end_block: Some(0),
                default_item_settings: item_settings,
            };

            let config = CollectionConfig {
                settings: CollectionSettings(BitFlags::from(CollectionSetting::TransferableItems)),  
                max_supply: None,
                mint_settings,
            };
            pop_api::nfts::create(admin, config)?;
			ink::env::debug_println!("Nfts::create_nft_collection: collection created successfully.");
            Ok(())
		}
    }

    #[cfg(test)]
	mod tests {
		use super::*;

		#[ink::test]
		fn default_works() {
			MyNfts::new();
		}
	}
}
