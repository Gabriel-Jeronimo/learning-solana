use std::u8;

use anchor_lang::prelude::*;

declare_id!("HjyTKMvem4vQdiT4tp1PWFRyAqmFjLZuduqrUBXccGeH");


#[program]
pub mod tiny_adventure {
    use anchor_lang::{solana_program::native_token::LAMPORTS_PER_SOL, system_program::{self, Transfer}};
    use super::*;

    const REWARD_AMOUNT: u64 = LAMPORTS_PER_SOL / 10;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.new_game_data_account.player_position = 0;
        print_player(ctx.accounts.new_game_data_account.player_position);
        Ok(())
    }

    pub fn reset_chest(ctx: Context<ResetChest>) -> Result<()> {
        ctx.accounts.game_data_account.player_position = 0;
        
        // What is exactly an accountinfo?
        let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer{ 
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.chest_reward.to_account_info()
        });

        system_program::transfer(cpi_context, REWARD_AMOUNT);
        Ok(())
    }

    pub fn move_left(ctx: Context<Move>) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account; 
        
        if (game_data_account.player_position == 0) {
            msg!("You are at the start.");
            return Ok(());
        }

        game_data_account.player_position -= 1;
        print_player(game_data_account.player_position);

        Ok(())
    }


    pub fn move_right(ctx: Context<Move>) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account; 
        
        if (game_data_account.player_position == 3) {
            msg!("You are at the end.");
            return Ok(());
        }

        game_data_account.player_position += 1;
        print_player(game_data_account.player_position);

        Ok(())
    }

}

fn print_player(player_position: u8) {
    match(player_position) {
        0 => {
            msg!("A journey begin!");
            msg!("o.......");
        },
        1 => msg!(".o......"),
        2 => msg!("..o....."),
        3 => {
            msg!("........\\o/");
            msg!("You have reached the end! Super!");
        },
        4..=u8::MAX => msg!("This position is beyond your kingdom.")
    }
}

#[account]
pub struct GameDataAccount {
    player_position: u8
}

#[account]
pub struct ChestRewardAccount {}

// Q: Whats the meaning of 'info?
// Q: Why signer must be mutable?
// Q: How to calculate the space? 
// Seed is fixed, meaning that anyone can use the same GameDataAccount
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init_if_needed, seeds=[b"level1"], bump, payer = signer, space = 8 + 1)]
    pub new_game_data_account: Account<'info, GameDataAccount>,
    #[account(init_if_needed, seeds=[b"reward"], bump, payer = signer, space = 8)]
    pub chest_reward: Account<'info, ChestRewardAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,

}

// Q: Why I need to have a signer here?
#[derive(Accounts)]
pub struct ResetChest<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
    #[account(mut)]
    pub chest_reward: Account<'info, ChestRewardAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>
}
// All instruction account structs need to derive Accounts
#[derive(Accounts)]
pub struct Move<'info> {
    #[account(mut)] 
    pub game_data_account: Account<'info, GameDataAccount>
}

