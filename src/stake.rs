#![no_std]

multiversx_sc::imports!();

mod storage;
pub mod model;
mod views;

use crate::model::*;

const ONE_DAY_IN_SECONDS: u64 = 86400;
const TOKEN_DECIMALS: u64 = 1;
const RESOURCE_DECIMALS: u64 = 1000000000000000000;

#[multiversx_sc::contract]
pub trait StakeContract: storage::StorageModule + views::ViewsModule {
    #[init]
    fn init(
        &self,
        collection: TokenIdentifier,
        eccu_id: TokenIdentifier,
        food_id: TokenIdentifier,
        beer_id: TokenIdentifier,
        wood_id: TokenIdentifier,
        stone_id: TokenIdentifier,
        iron_id: TokenIdentifier,
        wargear_id: TokenIdentifier,
    ) {
        self.collection().set(collection);
        self.eccu_identifier().set(eccu_id);
        self.food_identifier().set(food_id);
        self.beer_identifier().set(beer_id);
        self.wood_identifier().set(wood_id);
        self.stone_identifier().set(stone_id);
        self.iron_identifier().set(iron_id);
        self.wargear_identifier().set(wargear_id);
    }

    #[only_owner]
    #[endpoint(togglePause)]
    fn toggle_pause(&self) {
        self.pause().update(|pause| *pause = !*pause);
    }

    #[only_owner]
    #[endpoint(setSftsAllowed)]
    fn set_sfts_allowed(&self, sfts_allowed: MultiValueEncoded<u64>) {
        for sft_allowed in sfts_allowed.into_iter() {
            self.sfts_allowed().insert(sft_allowed);
        }
    }

    #[only_owner]
    #[endpoint(removeSftsAllowed)]
    fn remove_sfts_allowed(&self, sfts_removed: MultiValueEncoded<u64>) {
        for sft_removed in sfts_removed.into_iter() {
            self.sfts_allowed().remove(&sft_removed);
        }
    }

    #[only_owner]
    #[endpoint(setSftEccu)]
    fn set_sft_eccu(&self, nonce: u64, amount: BigUint) {
        require!(amount > BigUint::zero(), "Invalid amount!");

        self.sft_eccu(&nonce).set(amount);
    }

    #[only_owner]
    #[endpoint(setSftResource)]
    fn set_sft_resource(&self, nonce: u64, amount: BigUint) {
        require!(amount > BigUint::zero(), "Invalid amount!");

        self.sft_resource(&nonce).set(amount);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(eccuFund)]
    fn eccu_fund(&self) {
        let payment = self.call_value().single_esdt();
        require!(self.eccu_identifier().get() == payment.token_identifier, "Invalid token!");
        require!(payment.amount > BigUint::zero(), "Amount must be greater than zero!");

        self.eccu_amount().update(|amount| *amount += payment.amount);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(foodFund)]
    fn food_fund(&self) {
        let payment = self.call_value().single_esdt();
        require!(self.food_identifier().get() == payment.token_identifier, "Invalid token!");
        require!(payment.amount > BigUint::zero(), "Amount must be greater than zero!");

        self.food_amount().update(|amount| *amount += payment.amount);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(beerFund)]
    fn beer_fund(&self) {
        let payment = self.call_value().single_esdt();
        require!(self.beer_identifier().get() == payment.token_identifier, "Invalid token!");
        require!(payment.amount > BigUint::zero(), "Amount must be greater than zero!");

        self.beer_amount().update(|amount| *amount += payment.amount);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(woodFund)]
    fn wood_fund(&self) {
        let payment = self.call_value().single_esdt();
        require!(self.wood_identifier().get() == payment.token_identifier, "Invalid token!");
        require!(payment.amount > BigUint::zero(), "Amount must be greater than zero!");

        self.wood_amount().update(|amount| *amount += payment.amount);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(stoneFund)]
    fn stone_fund(&self) {
        let payment = self.call_value().single_esdt();
        require!(self.stone_identifier().get() == payment.token_identifier, "Invalid token!");
        require!(payment.amount > BigUint::zero(), "Amount must be greater than zero!");

        self.stone_amount().update(|amount| *amount += payment.amount);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(ironFund)]
    fn iron_fund(&self) {
        let payment = self.call_value().single_esdt();
        require!(self.iron_identifier().get() == payment.token_identifier, "Invalid token!");
        require!(payment.amount > BigUint::zero(), "Amount must be greater than zero!");

        self.iron_amount().update(|amount| *amount += payment.amount);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(wargearFund)]
    fn wargear_fund(&self) {
        let payment = self.call_value().single_esdt();
        require!(self.wargear_identifier().get() == payment.token_identifier, "Invalid token!");
        require!(payment.amount > BigUint::zero(), "Amount must be greater than zero!");

        self.wargear_amount().update(|amount| *amount += payment.amount);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(withdrawFunds)]
    fn withdraw(&self, withdraw_amount: BigUint) {
        let caller = self.blockchain().get_caller();
        require!(withdraw_amount > BigUint::zero(), "Amount must be greater than zero!");

        self.eccu_amount().update(|amount| *amount -= &withdraw_amount);
        self.send().direct_esdt(&caller, &self.eccu_identifier().get(), 0, &withdraw_amount);
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        require!(!self.pause().get(), "The stake is stopped!");
        let eccu_identifier = self.call_value().single_esdt();
        require!(self.collection().get() == eccu_identifier.token_identifier, "Invalid identifier!");
        require!(self.sfts_allowed().contains(&eccu_identifier.token_nonce), "Invalid sft nonce!");

        let caller = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();

        if self.sfts_staked(&caller).contains(&eccu_identifier.token_nonce) {
            self.calculate_rewards_and_save(eccu_identifier.token_nonce, &caller, &self.sft_staked_amount(&caller, &eccu_identifier.token_nonce).get());
            self.sft_staked_at(&caller, &eccu_identifier.token_nonce).set(current_time);
            self.sft_staked_amount(&caller, &eccu_identifier.token_nonce).update(|amount| *amount += eccu_identifier.amount);
        } else {
            self.sfts_staked(&caller).insert(eccu_identifier.token_nonce);
            self.sft_staked_amount(&caller, &eccu_identifier.token_nonce).set(eccu_identifier.amount);
            self.sft_staked_at(&caller, &eccu_identifier.token_nonce).set(current_time);
        }

        if !self.users_staked().contains(&caller) {
            self.users_staked().insert(caller);
        }
    }
    
    #[endpoint(unStake)]
    fn un_stake(&self, token_identifier: TokenIdentifier, nonce: u64, amount: BigUint) {
        require!(!self.pause().get(), "The stake is stopped!");
        require!(self.collection().get() == token_identifier, "Invalid identifier!");

        let caller = self.blockchain().get_caller();
        require!(self.sfts_staked(&caller).contains(&nonce), "You don't have this sft at stake!");
        require!(self.sft_staked_amount(&caller, &nonce).get() >= amount, "You don't have enough sfts at stake!");

        self.calculate_rewards_and_save(nonce, &caller, &BigUint::from(self.sft_staked_amount(&caller, &nonce).get()));
        let amount_staked = self.sft_staked_amount(&caller, &nonce).get();

        if amount_staked - &amount > BigUint::zero() {
            self.sft_staked_amount(&caller, &nonce).update(|amount_on_stake| *amount_on_stake -= amount.clone());
            self.sft_staked_at(&caller, &nonce).set(self.blockchain().get_block_timestamp());
        } else {
            self.sft_staked_amount(&caller, &nonce).clear();
            self.sft_staked_at(&caller, &nonce).clear();
            self.sfts_staked(&caller).remove(&nonce);
        }

        if self.sfts_staked(&caller).len() == 0 {
            self.users_staked().remove(&caller);
        }
        self.send().direct_esdt(&caller, &token_identifier, nonce, &amount);
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        let caller = self.blockchain().get_caller();
        let tokens = self.calculate_rewards(&caller);

        require!(tokens.len() > 0, "You don't have rewards!");

        self.reset_sfts_staked_time(&caller);

        for token in tokens.iter() {
            match token.name {
                TokenType::Eccu => {
                    // require!() sa adaug pentru toate in cazul in care nu sunt fonduri
                    if token.amount > BigUint::zero() {
                        let reward = token.amount * BigUint::from(TOKEN_DECIMALS);
                        self.send().direct_esdt(&caller, &self.eccu_identifier().get(), 0, &reward);
                        self.user_eccu(&caller).set(BigUint::zero());
                        self.eccu_amount().update(|amount| *amount -= reward);
                    }
                },
                TokenType::Food => {
                    if token.amount > BigUint::zero() {
                        let reward = token.amount * BigUint::from(RESOURCE_DECIMALS);
                        self.send().direct_esdt(&caller, &self.food_identifier().get(), 0, &reward);
                        self.user_food(&caller).set(BigUint::zero());
                        self.food_amount().update(|amount| *amount -= reward);
                    }
                },
                TokenType::Beer => {
                    if token.amount > BigUint::zero() {
                        let reward = token.amount * BigUint::from(RESOURCE_DECIMALS);
                        self.send().direct_esdt(&caller, &self.beer_identifier().get(), 0, &reward);
                        self.user_beer(&caller).set(BigUint::zero());
                        self.beer_amount().update(|amount| *amount -= reward);
                    }
                },
                TokenType::Stone => {
                    if token.amount > BigUint::zero() {
                        let reward = token.amount * BigUint::from(RESOURCE_DECIMALS);
                        self.send().direct_esdt(&caller, &self.stone_identifier().get(), 0, &reward);
                        self.user_stone(&caller).set(BigUint::zero());
                        self.stone_amount().update(|amount| *amount -= reward);
                    }
                },
                TokenType::Wood => {
                    if token.amount > BigUint::zero() {
                        let reward = token.amount * BigUint::from(RESOURCE_DECIMALS);
                        self.send().direct_esdt(&caller, &self.wood_identifier().get(), 0, &reward);
                        self.user_wood(&caller).set(BigUint::zero());
                        self.wood_amount().update(|amount| *amount -= reward);
                    }
                },
                TokenType::Iron => {
                    if token.amount > BigUint::zero() {
                        let reward = token.amount * BigUint::from(RESOURCE_DECIMALS);
                        self.send().direct_esdt(&caller, &self.iron_identifier().get(), 0, &reward);
                        self.user_iron(&caller).set(BigUint::zero());
                        self.iron_amount().update(|amount| *amount -= reward);
                    }
                },
                TokenType::Wargear => {
                    if token.amount > BigUint::zero() {
                        let reward = token.amount * BigUint::from(RESOURCE_DECIMALS);
                        self.send().direct_esdt(&caller, &self.wargear_identifier().get(), 0, &reward);
                        self.user_wargear(&caller).set(BigUint::zero());
                        self.wargear_amount().update(|amount| *amount -= reward);
                    }
                },
                _ => {}
            }
        }
    }

    fn calculate_rewards_and_save(
        &self,
        nonce: u64,
        address: &ManagedAddress,
        amount_to_unstake: &BigUint
    ) {
        let tokens = self.calculate_rewards_for_a_specific_sft(nonce, address, amount_to_unstake);

        for token in tokens.iter() {
            if token.name == TokenType::Food {
                self.user_food(address).update(|amount| *amount += token.amount.clone());
            }

            if token.name == TokenType::Beer {
                self.user_beer(address).update(|amount| *amount += token.amount.clone());
            }

            if token.name == TokenType::Wood {
                self.user_wood(address).update(|amount| *amount += token.amount.clone());
            }

            if token.name == TokenType::Stone {
                self.user_stone(address).update(|amount| *amount += token.amount.clone());
            }

            if token.name == TokenType::Iron {
                self.user_iron(address).update(|amount| *amount += token.amount.clone());
            }

            if token.name == TokenType::Wargear {
                self.user_wargear(address).update(|amount| *amount += token.amount.clone());
            }
            
            if token.name == TokenType::Eccu {
                self.user_eccu(address).update(|amount| *amount += token.amount.clone());
            }
        }
    }
    #[view(calculateForAnSpecific)]
    fn calculate_rewards_for_a_specific_sft(
        &self,
        nonce: u64,
        address: &ManagedAddress,
        amount_to_unstake: &BigUint
    ) -> ManagedVec<TokenReward<Self::Api>> {
        let staked_at = self.sft_staked_at(address, &nonce).get();
        let current_time = self.blockchain().get_block_timestamp();
        let sft_eccu = self.sft_eccu(&nonce).get();
        let sft_resource = self.sft_resource(&nonce).get();

        let days_staked = (current_time - staked_at) / ONE_DAY_IN_SECONDS;
        let mut tokens = ManagedVec::new();

        if days_staked > 0u64 {
            let actual_reward = BigUint::from(days_staked) * amount_to_unstake;

            match nonce {
                2 | 19 | 23 | 29 => {
                    tokens.push(
                        TokenReward::new(TokenType::Food, sft_resource * actual_reward.clone())
                    );
                    tokens.push(
                        TokenReward::new(TokenType::Eccu, sft_eccu * actual_reward)
                    );
                },
                35 | 41 | 49 | 55 => {
                    tokens.push(
                        TokenReward::new(TokenType::Food, sft_resource * actual_reward.clone())
                    );
                },
                37 | 43 | 51 | 57 => {
                    tokens.push(
                        TokenReward::new(TokenType::Wood, sft_resource * actual_reward.clone())
                    );
                },
                6 | 17 | 24 | 30 => {
                    tokens.push(
                        TokenReward::new(TokenType::Beer, sft_resource * actual_reward.clone())
                    );
                    tokens.push(
                        TokenReward::new(TokenType::Eccu, sft_eccu * actual_reward)
                    );
                },
                34 | 40 | 47 | 54 => {
                    tokens.push(
                        TokenReward::new(TokenType::Beer, sft_resource * actual_reward.clone())
                    );
                },
                3 | 16 | 22 | 27 => {
                    tokens.push(
                        TokenReward::new(TokenType::Iron, sft_resource * actual_reward.clone())
                    );
                    tokens.push(
                        TokenReward::new(TokenType::Eccu, sft_eccu * actual_reward)
                    );
                },
                36 | 42 | 50 | 56 => {
                    tokens.push(
                        TokenReward::new(TokenType::Iron, sft_resource * actual_reward.clone())
                    );
                },
                4 | 18 | 25 | 31 => {
                    tokens.push(
                        TokenReward::new(TokenType::Wargear, sft_resource * actual_reward.clone())
                    );
                    tokens.push(
                        TokenReward::new(TokenType::Eccu, sft_eccu * actual_reward)
                    );
                },
                33 | 39 | 46 | 53 => {
                    tokens.push(
                        TokenReward::new(TokenType::Wargear, sft_resource * actual_reward.clone())
                    );
                },
                38 | 44 | 52 | 58 => {
                    tokens.push(
                        TokenReward::new(TokenType::Stone, sft_resource * actual_reward.clone())
                    );
                },
                5 | 7 | 14 | 15 | 20 | 21 |26 | 28 => {
                    tokens.push(
                        TokenReward::new(TokenType::Eccu, sft_eccu * actual_reward)
                    );
                },
                _ => {}
            };
        }
        tokens
    }

    #[view(calculateReward)]
    fn calculate_rewards(
        &self,
        address: &ManagedAddress,
    ) -> ManagedVec<TokenReward<Self::Api>> {
        let mut tokens = ManagedVec::new();
        let user_eccu = self.user_eccu(address).get();
        let user_food = self.user_food(address).get();
        let user_beer = self.user_beer(address).get();
        let user_wood = self.user_wood(address).get();
        let user_stone = self.user_stone(address).get();
        let user_iron = self.user_iron(address).get();
        let user_wargear = self.user_wargear(address).get();

        tokens.push(TokenReward::new(TokenType::Eccu, user_eccu));
        tokens.push(TokenReward::new(TokenType::Food, user_food));
        tokens.push(TokenReward::new(TokenType::Beer, user_beer));
        tokens.push(TokenReward::new(TokenType::Wood, user_wood));
        tokens.push(TokenReward::new(TokenType::Stone, user_stone));
        tokens.push(TokenReward::new(TokenType::Iron, user_iron));
        tokens.push(TokenReward::new(TokenType::Wargear, user_wargear));

        for nonce in self.sfts_staked(&address).iter() {
            let sft_amount = self.sft_staked_amount(address, &nonce).get();
            let sft_tokens = self.calculate_rewards_for_a_specific_sft(nonce, address, &sft_amount);

            for token in sft_tokens.iter() {
                for mut token_returned in tokens.iter() {
                    if token_returned.name == token.name {
                        token_returned.amount += token.amount.clone();
                    }
                }
            }
        }

        tokens
    }

    fn reset_sfts_staked_time(&self, address: &ManagedAddress) {
        let current_time = self.blockchain().get_block_timestamp();

        for sft in self.sfts_staked(address).iter() {
            self.sft_staked_at(address, &sft).set(current_time);
        }
    }
}