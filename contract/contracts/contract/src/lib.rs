#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env
};

#[contracttype]
#[derive(Clone)]
pub struct Campaign {
    pub creator: Address,
    pub goal: i128,
    pub pledged: i128,
    pub deadline: u64,
    pub claimed: bool,
}

#[contracttype]
pub enum DataKey {
    Campaign(u32),
    CampaignCount,
    Pledge(u32, Address),
}

#[contract]
pub struct CrowdfundingContract;

#[contractimpl]
impl CrowdfundingContract {

    // Create a new campaign
    pub fn create_campaign(
        env: Env,
        creator: Address,
        goal: i128,
        duration: u64,
    ) -> u32 {
        creator.require_auth();

        let count: u32 = env.storage().instance()
            .get(&DataKey::CampaignCount)
            .unwrap_or(0);

        let campaign = Campaign {
            creator: creator.clone(),
            goal,
            pledged: 0,
            deadline: env.ledger().timestamp() + duration,
            claimed: false,
        };

        env.storage().instance().set(&DataKey::Campaign(count), &campaign);
        env.storage().instance().set(&DataKey::CampaignCount, &(count + 1));

        count
    }

    // Pledge funds
    pub fn pledge(env: Env, id: u32, from: Address, amount: i128) {
        from.require_auth();

        let mut campaign: Campaign = env.storage().instance()
            .get(&DataKey::Campaign(id))
            .expect("Campaign not found");

        if env.ledger().timestamp() > campaign.deadline {
            panic!("Campaign ended");
        }

        campaign.pledged += amount;

        let prev: i128 = env.storage().instance()
            .get(&DataKey::Pledge(id, from.clone()))
            .unwrap_or(0);

        env.storage().instance()
            .set(&DataKey::Pledge(id, from), &(prev + amount));

        env.storage().instance().set(&DataKey::Campaign(id), &campaign);
    }

    // Claim funds if goal reached
    pub fn claim(env: Env, id: u32, creator: Address) {
        creator.require_auth();

        let mut campaign: Campaign = env.storage().instance()
            .get(&DataKey::Campaign(id))
            .expect("Campaign not found");

        if creator != campaign.creator {
            panic!("Not creator");
        }

        if campaign.pledged < campaign.goal {
            panic!("Goal not reached");
        }

        if campaign.claimed {
            panic!("Already claimed");
        }

        campaign.claimed = true;

        env.storage().instance().set(&DataKey::Campaign(id), &campaign);
    }

    // Refund if goal not reached
    pub fn refund(env: Env, id: u32, user: Address) {
        user.require_auth();

        let campaign: Campaign = env.storage().instance()
            .get(&DataKey::Campaign(id))
            .expect("Campaign not found");

        if env.ledger().timestamp() <= campaign.deadline {
            panic!("Campaign still active");
        }

        if campaign.pledged >= campaign.goal {
            panic!("Goal was reached");
        }

        let pledged: i128 = env.storage().instance()
            .get(&DataKey::Pledge(id, user.clone()))
            .unwrap_or(0);

        if pledged <= 0 {
            panic!("No funds to refund");
        }

        env.storage().instance()
            .set(&DataKey::Pledge(id, user), &0);
    }

    // View campaign
    pub fn get_campaign(env: Env, id: u32) -> Campaign {
        env.storage().instance()
            .get(&DataKey::Campaign(id))
            .expect("Campaign not found")
    }
}