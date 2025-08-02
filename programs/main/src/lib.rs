use std::convert::AsRef;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("A6kupDWJAzffm1J3cKVNRBLL3zuNT7cXFFgeMxDPZ3pc");

#[program]
pub mod main {
    use super::*;

    pub fn initialize_escrow(ctx: Context<InitializeEscrow>, amount: u64) -> Result<()> {
        let sol_vault = &mut ctx.accounts.sol_vault;
        let signer = &mut ctx.accounts.signer;
        let system_program = &ctx.accounts.system_program;

        let vault_balance_before = sol_vault.get_lamports();

        transfer(
           CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: signer.to_account_info(),
                    to: sol_vault.to_account_info(),
                },
           ),
           amount,
        )?;

        let vault_balance_after = sol_vault.get_lamports();

        require_eq!(vault_balance_before + amount, vault_balance_after);

        Ok(())
    }

    pub fn confirm_delivery(ctx: Context<ConfirmDelivery>, amount: u64) -> Result<()> {
        let sol_vault = &mut ctx.accounts.sol_vault;
        let _signer = &mut ctx.accounts.signer;
        let recipient  = &mut ctx.accounts.recipient;
        let system_program = &ctx.accounts.system_program;

        let vault_balance_before = sol_vault.get_lamports();

        let bump = &[ctx.bumps.sol_vault];
        let seeds = &[b"vault".as_ref(), bump];
        let signer_seeds = &[&seeds[..]];

        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: sol_vault.to_account_info(),
                    to: recipient.to_account_info(),
                },
            ).with_signer(signer_seeds),
            amount,
        )?;

        let vault_balance_after = sol_vault.get_lamports();

        require_eq!(vault_balance_before - amount, vault_balance_after);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(
        mut,
        seeds=[b"vault".as_ref()],
        bump,
    )]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmDelivery<'info> {
    #[account(
        mut,
        seeds=[b"vault".as_ref()],
        bump
    )]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
