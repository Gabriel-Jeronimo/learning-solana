use anchor_lang::prelude::*;

declare_id!("7DYWSYoDziALSfAt7wq5zjmSfUo1edQMjpoxULKypBJu");

#[program]
pub mod deposit_withdraw {
    use anchor_lang::{solana_program::native_token::LAMPORTS_PER_SOL, system_program::{self, Transfer}};

    use super::*;


    const MIN_DEPOSIT: u64 = LAMPORTS_PER_SOL / 10;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Your account was created successfuly: {}", ctx.accounts.vault.key());
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        if amount < MIN_DEPOSIT {
            return err!(Errors::DepositBelowMinimum);
        }

        let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer{
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.vault.to_account_info()
        });

        system_program::transfer(cpi_context, amount);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer{
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.signer.to_account_info(),
        });

        // Q: How should I handle transfer exceptions? What is the Solana pattern?
        system_program::transfer(cpi_context, amount);
        Ok(())
    }

}

#[account]
pub struct Vault {}
 
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init_if_needed, seeds=[b"vault", signer.key().as_ref()], bump, payer = signer, space = 8)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub signer: Signer<'info>
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}


#[error_code]
pub enum Errors {
    #[msg("The minimum deposit is 0.1 SOL")]
    DepositBelowMinimum
}