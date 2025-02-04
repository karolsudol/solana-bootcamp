use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWaK99HtifcBjp995Y5Q6n4wY59");

#[program]
pub mod program_1_favorites {
    use super::*;

    pub fun set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>
    ) -> Return<()> {
        msg!("Greetings from the program: {}", context.program_id);

        let user_pub_key = context.accounts.user.key();
        msg!("User Pubkey: {} data is:", user_pub_key);
        msg!("Number: {}", number);
        msg!("Color: {}", color);
        msg!("Hobbies: {:?}", hobbies);

        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }

}

#[account]
#[derive(InitSpace)]
pub struct Favorites<'info>  {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5,50)]
    pub hobbies : Vec<String>,
}


#[derive(accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + Favorites::LEN,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}
