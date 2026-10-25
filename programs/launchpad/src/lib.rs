// Ada Lovelace - Solana Autonomous Launchpad Program
// Token Symbol: $LOVELACE
// Architecture: Solana Sealevel SVM & SPL Token Bonding Curve

pub struct LaunchpadState {
    pub is_initialized: bool,
    pub authority: [u8; 32],
    pub total_supply: u64,
    pub reserve_lamports: u64,
    pub initial_price_lamports: u64,
    pub curve_slope_lamports: u64,
}

impl LaunchpadState {
    pub const INITIAL_PRICE: u64 = 100_000; // 0.0001 SOL in lamports
    pub const SLOPE: u64 = 10;

    pub fn new(authority: [u8; 32]) -> Self {
        Self {
            is_initialized: true,
            authority,
            total_supply: 0,
            reserve_lamports: 0,
            initial_price_lamports: Self::INITIAL_PRICE,
            curve_slope_lamports: Self::SLOPE,
        }
    }

    pub fn calculate_mint_tokens(&self, lamports_paid: u64) -> u64 {
        let current_price = self.initial_price_lamports + (self.curve_slope_lamports * (self.total_supply / 1_000_000_000));
        (lamports_paid * 1_000_000_000) / current_price
    }

    pub fn disburse_milestone(&mut self, amount_lamports: u64) -> bool {
        if amount_lamports <= self.reserve_lamports {
            self.reserve_lamports -= amount_lamports;
            true
        } else {
            false
        }
    }
}
