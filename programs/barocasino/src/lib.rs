use anchor_lang::prelude::*;

// Your program Id will be added here when you enter "build" command
declare_id!("3HuN3uYw1h33gUyJwoy1vRaNLEijd782ceLQPgSCk9b4");

#[error_code]
pub enum ErrorCode {
    #[msg("Balance Not Enough to bet")]
    BalanceNotEnough,
    // Add other variants if needed
}

const INIT_BALANCE: u64 = 10000;
const BANKER_WIN_RATE: u64 = 1950;
const PLAYER_WIN_RATE: u64 = 2000;
const TIE_RATE: u64 = 8000;
const RATE_DECIMAL: u64 = 1000;

#[program]
pub mod barocasino {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        let bet_account: &mut Account<'_, BetAccount> = &mut ctx.accounts.bet_account;
        bet_account.owner = *ctx.accounts.authority.key;
        bet_account.balance = INIT_BALANCE;

        Ok(())
    }

    pub fn bet(
        ctx: Context<Bet>,
        banker_win: u64,
        player_win: u64,
        tie: u64,
        banker_pair: u64,
        player_pair: u64,
    ) -> Result<()> {
        let bet_account: &mut Account<'_, BetAccount> = &mut ctx.accounts.bet_account;
        let total_bet = banker_win + player_win + tie + banker_pair + player_pair;
        if bet_account.balance < total_bet {
            return Err(ErrorCode::BalanceNotEnough.into());
        }
        bet_account.balance -= total_bet;
        // Get current slot
        let slot = Clock::get()?.slot;
        // Generate pseudo-random number using XORShift with the current slot as seed
        let xorshift_output = xorshift64(slot);
        // Calculate random damage
        let random_number = xorshift_output % 10000;

        // Calculate the result of the game
        let game_outcome = calculate_game_outcome(random_number);
        let payback = match game_outcome {
            GameOutcome::BankerWin => banker_win * BANKER_WIN_RATE / RATE_DECIMAL,
            GameOutcome::PlayerWin => player_win * PLAYER_WIN_RATE / RATE_DECIMAL,
            GameOutcome::Tie => tie * TIE_RATE / RATE_DECIMAL,
        };

        bet_account.balance += payback;
        let result = match game_outcome {
            GameOutcome::BankerWin => "Banker Win",
            GameOutcome::PlayerWin => "Player Win",
            GameOutcome::Tie => "Tie",
        };
        emit!(GameResult {
            bettor: *ctx.accounts.authority.key,
            banker_win: banker_win,
            player_win: player_win,
            tie: tie,
            banker_pair: banker_pair,
            player_pair: player_pair,
            payback: payback,
            result: result.to_string()
        });
        Ok(())
    }
}

fn calculate_game_outcome(random_number: u64) -> GameOutcome {
    if random_number < 4586 {
        GameOutcome::BankerWin
    } else if random_number >= 4586 && random_number < 4586 + 4462 {
        GameOutcome::PlayerWin
    } else {
        GameOutcome::Tie
    }
}

fn xorshift64(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x << 21;
    x ^= x >> 35;
    x ^= x << 4; 
    x
}

#[derive(Accounts)]
#[instruction()]
pub struct Init<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + 8 + 32+ (4 + 12)+ 8 + 1,
        seeds = [b"bet", authority.key().as_ref()], 
        bump
    )]
    pub bet_account: Account<'info, BetAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(banker_win : u64, player_win : u64, tie : u64, banker_pair : u64, player_pair : u64)]
pub struct Bet<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"bet", authority.key().as_ref()],
        bump
    )]
    pub bet_account: Account<'info, BetAccount>,
}

#[account]
#[derive(Default)]
pub struct BetAccount {
    pub owner: Pubkey,
    pub balance: u64,
}

#[event]
pub struct GameResult {
    pub bettor: Pubkey,
    pub player_win: u64,
    pub banker_win: u64,
    pub tie: u64,
    pub banker_pair: u64,
    pub player_pair: u64,
    pub payback: u64,
    pub result: String,
}

enum GameOutcome {
    BankerWin,
    PlayerWin,
    Tie,
}
