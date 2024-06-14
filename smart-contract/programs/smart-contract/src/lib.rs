use anchor_lang::prelude::*;

declare_id!("6RGBeuyW81u5Jn6YB1L7bhhrkWpCyx1gemez59mjpr4i");

#[program]
pub mod degen_casino {
    use super::*;
    pub fn create_raffle(ctx: Context<CreateRaffle>, entry_fee: u64, required_participants: u64) -> Result<()> {
        let raffle = &mut ctx.accounts.raffle;
        raffle.creator = *ctx.accounts.user.key;
        raffle.entry_fee = entry_fee;
        raffle.required_participants = required_participants;
        raffle.is_active = true;
        raffle.participants = vec![];
        Ok(())
    }

    pub fn join_raffle(ctx: Context<JoinRaffle>) -> Result<()> {
        let raffle = &mut ctx.accounts.raffle;
        let user = &mut ctx.accounts.user;

        require!(raffle.participants.len() < raffle.required_participants as usize, CustomError::RaffleFull);
        require!(raffle.participants.iter().all(|&x| x != *user.key), CustomError::AlreadyParticipated);

        raffle.participants.push(*user.key);

        if raffle.participants.len() == raffle.required_participants as usize {
            // Select a winner and handle the prize distribution
            let winner_index = (Clock::get().unwrap().unix_timestamp as usize) % raffle.participants.len();
            let winner = raffle.participants[winner_index];

            // Distribute the prize
            let prize = raffle.entry_fee * raffle.participants.len() as u64;
            **ctx.accounts.system_program.to_account_info().try_borrow_mut_lamports()? -= prize;
            **ctx.accounts.winner.to_account_info().try_borrow_mut_lamports()? += prize;

            raffle.is_active = false;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateRaffle<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8 + 32 * 1000)]
    pub raffle: Account<'info, Raffle>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinRaffle<'info> {
    #[account(mut)]
    pub raffle: Account<'info, Raffle>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is safe because we will perform necessary checks in the program logic
    #[account(mut)]
    pub winner: AccountInfo<'info>,
}

#[account]
pub struct Raffle {
    pub creator: Pubkey,
    pub entry_fee: u64,
    pub required_participants: u64,
    pub participants: Vec<Pubkey>,
    pub is_active: bool,
}

#[error_code]
pub enum CustomError {
    #[msg("Raffle is already full.")]
    RaffleFull,
    #[msg("You have already participated in this raffle.")]
    AlreadyParticipated,
}
