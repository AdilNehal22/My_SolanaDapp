use anchor_lang::prelude::*;

declare_id!("6Co28kyQJfuySddKCNjJ2Jgt4tVWSqLExZvrSoXcpudx");

// In this program we have two functions - create and increment. These two functions are the RPC request handlers that we will be able to call from a client application to interact with the program.

#[program]
pub mod mysolanaapp {

    use super::*;

    // The first parameter of an RPC handler is the Context struct, which describes the context that will be passed in when the function is called and how to handle it. In the case of Create, we are expecting three parameters: base_account, user, and system_program.
    
    pub fn create(ctx: Context<Create>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
    
}

// Transaction instructions

#[derive(Accounts)]
pub struct Create<'info> {
    // The #[account(...)] attributes define constraints and instructions that are related to the proceeding account where declared. If any of these constraints do not hold, then the instruction will never execute.
    #[account(init, payer = user, space = 16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}
//Any client calling this program with the proper base_account can call these RPC methods.
// Transaction instructions
#[derive(Accounts)]
pub struct Increment<'info>{
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// An account that goes inside a transaction instruction
#[account]
pub struct BaseAccount {
    pub count: u64,
}



