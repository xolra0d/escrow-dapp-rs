use std::convert::AsRef;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("A6kupDWJAzffm1J3cKVNRBLL3zuNT7cXFFgeMxDPZ3pc");

#[program]
pub mod main {
    use super::*;

    pub fn initialize_escrow(ctx: Context<InitializeEscrow>, deal_info: DealInfo) -> Result<()> {
        if deal_info.title.len() > 128 {
            return err!(ErrorCode::TitleTooLong)
        }
        let escrow_account = &mut ctx.accounts.escrow_account;
        let signer = &mut ctx.accounts.signer;
        let system_program = &ctx.accounts.system_program;

        let vault_balance_before = escrow_account.get_lamports();

        escrow_account.title = deal_info.title;
        escrow_account.seller = deal_info.seller;
        escrow_account.buyer = deal_info.buyer;
        escrow_account.bump = ctx.bumps.escrow_account;
        escrow_account.lamports = deal_info.lamports;

        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: signer.to_account_info(),
                    to: escrow_account.to_account_info(),
                },
           ),
            deal_info.lamports,
        )?;

        let vault_balance_after = escrow_account.get_lamports();

        require_eq!(vault_balance_before + deal_info.lamports, vault_balance_after);

        Ok(())
    }

    pub fn confirm_delivery(ctx: Context<ConfirmDelivery>) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        let recipient  = &mut ctx.accounts.recipient;

        let amount = escrow_account.lamports;

        **escrow_account.to_account_info().try_borrow_mut_lamports()? -= amount;
        **recipient.to_account_info().try_borrow_mut_lamports()? += amount;


        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DealInfo {
    title: String, // max 128 chars
    lamports: u64,
    seller: Pubkey,
    buyer: Pubkey,
}

#[account]
pub struct EscrowAccount {
    pub title: String,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub lamports: u64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"vault".as_ref()],
        bump,
        space = 8 + (4 + 128) + 8 + 32 + 32 + 8 + 1,
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmDelivery<'info> {
    #[account(
        mut,
        seeds=[b"vault".as_ref()],
        bump = escrow_account.bump,
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[error_code]
pub enum ErrorCode {
    #[msg("Title should be less than or equal to 128 chars")]
    TitleTooLong,
}