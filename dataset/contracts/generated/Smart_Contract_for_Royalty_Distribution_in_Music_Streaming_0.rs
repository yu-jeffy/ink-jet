
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod music_royalties {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct MusicRoyalties {
        /// Mapping from song ID to its royalty information.
        royalties: Mapping<u32, RoyaltyInfo>,
        /// The admin account that can add songs and update royalty information.
        admin: AccountId,
    }

    /// The royalty information for a song.
    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct RoyaltyInfo {
        /// Total amount of collected royalties for the song.
        total_collected: Balance,
        /// Mapping from collaborator account to their share (in basis points, so 10,000 is 100%).
        shares: Mapping<AccountId, u16>,
        /// Mapping from collaborator account to their collected amount.
        collected: Mapping<AccountId, Balance>,
    }

    #[ink(event)]
    pub struct RoyaltiesCollected {
        #[ink(topic)]
        song_id: u32,
        amount: Balance,
    }

    #[ink(event)]
    pub struct RoyaltiesDistributed {
        #[ink(topic)]
        song_id: u32,
        recipient: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct SongAdded {
        #[ink(topic)]
        song_id: u32,
    }

    #[ink(event)]
    pub struct SharesUpdated {
        #[ink(topic)]
        song_id: u32,
        collaborator: AccountId,
        shares: u16,
    }

    impl MusicRoyalties {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            Self {
                admin,
                ..Default::default()
            }
        }

        #[ink(message)]
        pub fn add_song(&mut self, song_id: u32) {
            self.only_admin();
            self.royalties.insert(song_id, &RoyaltyInfo::default());
            self.env().emit_event(SongAdded { song_id });
        }

        #[ink(message)]
        pub fn update_shares(&mut self, song_id: u32, collaborator: AccountId, shares: u16) {
            self.only_admin();
            let mut info = self.royalties.get(song_id).expect("Song does not exist");
            info.shares.insert(collaborator, &shares);
            self.env().emit_event(SharesUpdated {
                song_id,
                collaborator,
                shares,
            });
        }

        #[ink(message, payable)]
        pub fn collect_royalties(&mut self, song_id: u32) {
            let amount = self.env().transferred_balance();
            let mut info = self.royalties.get(song_id).expect("Song does not exist");
            info.total_collected += amount;
            self.env().emit_event(RoyaltiesCollected { song_id, amount });
        }

        #[ink(message)]
        pub fn distribute_royalties(&mut self, song_id: u32) {
            let info = self.royalties.get(song_id).expect("Song does not exist");
            let total_shares: u32 = info.shares.iter().fold(0, |acc, (_, &s)| acc + s as u32);
            assert!(total_shares <= 10_000, "Total shares exceed 100%");

            for (collaborator, &shares) in info.shares.iter() {
                let amount = info.total_collected * shares as u128 / total_shares as u128;
                let collected = info.collected.get(collaborator).unwrap_or_default() + amount;
                info.collected.insert(collaborator, &collected);

                if amount > 0 {
                    self.env().transfer(collaborator, amount).expect("Transfer failed");
                    self.env().emit_event(RoyaltiesDistributed {
                        song_id,
                        recipient: collaborator,
                        amount,
                    });
                }
            }
        }

        fn only_admin(&self) {
            assert_eq!(self.env().caller(), self.admin, "Only admin can perform this action");
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{
            test,
            DefaultEnvironment,
        };
        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            let contract = MusicRoyalties::new(accounts.alice);
            assert_eq!(contract.admin, accounts.alice);
        }

        #[ink::test]
        fn add_song_works() {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            let mut contract = MusicRoyalties::new(accounts.alice);
            contract.add_song(1);
            assert!(contract.royalties.contains(1));
        }

        #[ink::test]
        #[should_panic]
        fn add_song_non_admin_fails() {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            let mut contract = MusicRoyalties::new(accounts.alice);
            test::set_caller::<DefaultEnvironment>(accounts.bob);
            contract.add_song(1);
        }

        #[ink::test]
        fn update_shares_works() {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            let mut contract = MusicRoyalties::new(accounts.alice);
            contract.add_song(1);
            contract.update_shares(1, accounts.bob, 5000);
            let info = contract.royalties.get(1).unwrap();
            assert_eq!(info.shares.get(accounts.bob), Some(5000));
        }

        #[ink::test]
        #[should_panic]
        fn update_shares_non_admin_fails() {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            let mut contract = MusicRoyalties::new(accounts.alice);
            test::set_caller::<DefaultEnvironment>(accounts.bob);
            contract.update_shares(1, accounts.bob, 5000);
        }

        #[ink::test]
        fn collect_royalties_works() {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            let mut contract = MusicRoyalties::new(accounts.alice);
            contract.add_song(1);
            test::set_caller::<DefaultEnvironment>(accounts.bob);
            test::set_value_transferred::<DefaultEnvironment>(100);
            contract.collect_royalties(1);
            let info = contract.royalties.get(1).unwrap();
            assert_eq!(info.total_collected, 100);
        }

        #[ink::test]
        fn distribute_royalties_works() {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            let mut contract = MusicRoyalties::new(accounts.alice);
            contract.add_song(1);
            contract.update_shares(1, accounts.bob, 5000);
            contract.update_shares(1, accounts.eve, 5000);
            test::set_caller::<DefaultEnvironment>(accounts.bob);
            test::set_value_transferred::<DefaultEnvironment>(100);
            contract.collect_royalties(1);
            contract.distribute_royalties(1);
            let info = contract.royalties.get(1).unwrap();
            assert_eq!(info.collected.get(accounts.bob), Some(50));
            assert_eq!(info.collected.get(accounts.eve), Some(50));
        }
    }
}
