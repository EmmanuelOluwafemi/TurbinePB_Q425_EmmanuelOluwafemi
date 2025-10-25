use anchor_lang::prelude::*;

declare_id!("4MNfPWT7VsdPV4wifca4Xc7oGP5yt8U2zqkih8HNnESf");

// #[program]
// pub mod anchor_counter {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         let counter = &mut ctx.accounts.counter;
//         counter.count = 0;
//         msg!("Counter Account Created");
//         msg!("Current Count: { }", counter.count);
//         Ok(())
//     }

//     pub fn increment(ctx: Context<Update>) -> Result<()> {
//         let counter = &mut ctx.accounts.counter;
//         counter.count = counter.count.checked_add(1).unwrap();
//         msg!("Counter incremented. Current count: {}", counter.count);
//         Ok(())
//     }

//     pub fn decrement(ctx: Context<Update>) -> Result<()> {
//         let counter = &mut ctx.accounts.counter;
//         counter.count = counter.count.checked_sub(1).unwrap();
//         msg!("Counter incremented. Current count: {}", counter.count);
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(init,
//         payer = user,
//         size = DISCRIMINATOR + Counter::INIT_SPACE,
//         // seeds = [b"counter", user.key().as_ref()],
//         // bump
//     )]
//     pub counter: Account<'info, Counter>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct Update<'info> {
//     #[account(mut)]
//     pub counter: Account<'info, Counter>,
//     pub user: Signer<'info>,
// }

// #[account]
// #[derive(InitSpace)]
// pub struct Counter {
//     pub count: u64
// }

// const DISCRIMINATOR: usize = 8;

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter account created. Current count: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current count: {}", counter.count);
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.checked_sub(1).unwrap();
        msg!("Counter incremented. Current count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
        payer = user,
        space = DISCRIMINATOR + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
}

const DISCRIMINATOR: usize = 8;
