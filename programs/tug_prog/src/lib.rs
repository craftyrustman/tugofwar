use anchor_lang::prelude::*;

declare_id!("xRw1NFApY2terWNR2j3HHBfT8jgiKqjaEBtYnJ2B3Jh");

#[program]
pub mod tug_prog {
    use super::*;

    const MAX_POSITION: u16 = 20;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn restart_game(ctx: Context<Restart>) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account;

        if game_data_account.player_position > 0 && game_data_account.player_position < MAX_POSITION {
            panic!("you can't restart the game while it is still running!");
        }

        game_data_account.player_position = MAX_POSITION / 2;
        Ok(())
    }

    pub fn pull_left(ctx: Context<MoveLeft>) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account;
        
        if game_data_account.player_position <= 0 || game_data_account.player_position >= MAX_POSITION {
            panic!("The game is over, stop pulling left!");
        }

        if game_data_account.player_position <= 0 {
            msg!("Team Left won, Hurray!!! \\o/");
            display_game(game_data_account.player_position);
        } else {
            game_data_account.player_position -= 1;
            display_game(game_data_account.player_position);
        }
        Ok(())
    }

    pub fn pull_right(ctx: Context<MoveRight>) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account;

        if game_data_account.player_position <= 0 || game_data_account.player_position >= MAX_POSITION {
            panic!("The game is over, stop pulling right!");
        }

        if game_data_account.player_position >= MAX_POSITION {
            msg!("Team Right won, Hurray!!! \\o/");
            display_game(game_data_account.player_position);
        } else {
            game_data_account.player_position = game_data_account.player_position + 1;
            display_game(game_data_account.player_position);
        }
        Ok(())
    }
}

fn display_game(position: u16) -> &'static str{
    match position {
          0 => "\\o/-------|-------OOO____________________",
          1 => "_ooo-------|-------OOO___________________",
          2 => "__ooo-------|-------OOO__________________",
          3 => "___ooo-------|-------OOO_________________",
          4 => "____ooo-------|-------OOO________________",
          5 => "_____ooo-------|-------OOO_______________",
          6 => "______ooo-------|-------OOO______________",
          7 => "_______ooo-------|-------OOO_____________",
          8 => "________ooo-------|-------OOO____________",
          9 => "_________ooo-------|-------OOO___________",
         10 => "__________ooo-------|-------OOO__________",
         11 => "___________ooo-------|-------OOO_________",
         12 => "____________ooo-------|-------OOO________",
         13 => "_____________ooo-------|-------OOO_______",
         14 => "______________ooo-------|-------OOO______",
         15 => "_______________ooo-------|-------OOO_____",
         16 => "________________ooo-------|-------OOO____",
         17 => "_________________ooo-------|-------OOO___",
         18 => "__________________ooo-------|-------OOO__",
         19 => "___________________ooo-------|-------OOO_",
         20 => "____________________ooo-------|-------\\o/",
        _ => "",
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"tug_of_war"],
        bump,
        payer = signer,
        space = 8 + 2
    )]
    pub new_game_data_account: Account<'info, GameDataAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MoveLeft<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}

#[derive(Accounts)]
pub struct MoveRight<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}

#[derive(Accounts)]
pub struct Restart<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}

#[account]
pub struct GameDataAccount {
    pub player_position: u16,
}