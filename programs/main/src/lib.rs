use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("A6kupDWJAzffm1J3cKVNRBLL3zuNT7cXFFgeMxDPZ3pc");

#[program]
pub mod main {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, deal_info: DealInfo) -> Result<()> {
        require!(
            deal_info.title.len() <= EscrowAccount::MAX_TITLE_LENGTH,
            ErrorCode::TitleTooLong
        );

        let escrow_account = &mut ctx.accounts.escrow_account;

        escrow_account.title = deal_info.title;
        escrow_account.lamports = deal_info.lamports;
        escrow_account.buyer = ctx.accounts.buyer.key();
        escrow_account.seller = deal_info.seller;
        escrow_account.bump = ctx.bumps.escrow_account;

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.buyer.to_account_info(),
                    to: escrow_account.to_account_info(),
                },
            ),
            deal_info.lamports,
        )?;

        Ok(())
    }

    pub fn describe_escrow(ctx: Context<DescribeEscrow>) -> Result<()> {
        let escrow_account = &ctx.accounts.escrow_account;

        msg!("Title: {}", escrow_account.title);
        msg!("Seller: {}", escrow_account.seller);
        msg!("Buyer: {}", escrow_account.buyer);
        msg!("Lamports: {}", escrow_account.lamports);

        Ok(())
    }

    pub fn confirm_escrow(ctx: Context<ConfirmEscrow>) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        let seller  = &mut ctx.accounts.seller;

        escrow_account.sub_lamports(escrow_account.lamports)?;
        seller.add_lamports(escrow_account.lamports)?;

        Ok(())
    }

    pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        let buyer  = &mut ctx.accounts.buyer;

        escrow_account.sub_lamports(escrow_account.lamports)?;
        buyer.add_lamports(escrow_account.lamports)?;


        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DealInfo {
    title: String, // max 128 chars
    lamports: u64,
    seller: Pubkey,
}

#[account]
pub struct EscrowAccount {
    pub title: String,
    pub lamports: u64,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub bump: u8,
}

impl EscrowAccount {
    pub const MAX_TITLE_LENGTH: usize = 128;

    pub const MAX_SPACE: usize = 8 + // discriminator: u8, 8
        4 + Self::MAX_TITLE_LENGTH + // title String
        8 + // lamports: u64
        32 + // buyer: pubkey
        32 + // seller: pubkey
        1; // bump: u8
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(
        init,
        payer = buyer,
        seeds = [b"sol_vault".as_ref(), seller.key().as_ref(), buyer.key().as_ref()],
        bump,
        space = EscrowAccount::MAX_SPACE,
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: we only need pubkey
    pub seller: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DescribeEscrow<'info> {
    #[account(
        seeds = [b"sol_vault".as_ref(), seller.key().as_ref(), buyer.key().as_ref()],
        bump = escrow_account.bump,
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    /// CHECK: we only need pubkey
    pub buyer: AccountInfo<'info>,
    /// CHECK: we only need pubkey
    pub seller: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ConfirmEscrow<'info> {
    #[account(
        mut,
        close = buyer,
        has_one = buyer @ ErrorCode::Unauthorised,
        has_one = seller @ ErrorCode::WrongSeller,
        seeds=[b"sol_vault".as_ref(), seller.key().as_ref(), buyer.key().as_ref()],
        bump = escrow_account.bump,
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: we only need pubkey
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    #[account(
        mut,
        close = buyer,
        has_one = buyer @ ErrorCode::Unauthorised,
        has_one = seller @ ErrorCode::WrongSeller,
        seeds=[b"sol_vault".as_ref(), seller.key().as_ref(), buyer.key().as_ref()],
        bump = escrow_account.bump,
        constraint = signer.key() == escrow_account.buyer || signer.key() == escrow_account.seller @ ErrorCode::Unauthorised,
    )]
    pub escrow_account: Account<'info, EscrowAccount>,

    // #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: we only need pubkey
    #[account(mut)]
    pub buyer: AccountInfo<'info>,
    /// CHECK: we only need pubkey
    pub seller: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Title should be less than or equal to 128 chars")]
    TitleTooLong,
    #[msg("Seller account should match account provided in deal_info!")]
    WrongSeller,
    #[msg("Buyer account should match account provided in deal_info!")]
    WrongBuyer,
    #[msg("Not authorised!")]
    Unauthorised,
}
