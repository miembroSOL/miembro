//license PalaBlockchain

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
    sysvar::rent,
};

entrypoint!(_entrypoint);

pub fn _entrypoint(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let instruction = instruction_data[0];
    match instruction {
        0 => {
            msg!("Initialize");
            let ctx = InitializeContext {
                metadata: next_account_info(accounts_iter)?,
                authority: next_account_info(accounts_iter)?,
                mint: next_account_info(accounts_iter)?,
                rent: rent::Rent::from_account_info(next_account_info(accounts_iter)?)?,
                system_program: next_account_info(accounts_iter)?,
            };
            initialize(ctx)
        }
        1 => {
            msg!("TransferNFT");
            let ctx = TransferNFTContext {
                from: next_account_info(accounts_iter)?,
                to: next_account_info(accounts_iter)?,
                authority: next_account_info(accounts_iter)?,
                metadata: next_account_info(accounts_iter)?,
                system_program: next_account_info(accounts_iter)?,
            };
            transfer_nft(ctx)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

struct InitializeContext<'a, 'b> {
    metadata: &'a AccountInfo<'b>,
    authority: &'a AccountInfo<'b>,
    mint: &'a AccountInfo<'b>,
    rent: rent::Rent,
    system_program: &'a AccountInfo<'b>,
}

struct TransferNFTContext<'a, 'b> {
    from: &'a AccountInfo<'b>,
    to: &'a AccountInfo<'b>,
    authority: &'a AccountInfo<'b>,
    metadata: &'a AccountInfo<'b>,
    system_program: &'a AccountInfo<'b>,
}

fn initialize(ctx: InitializeContext) -> ProgramResult {
    let mint_authority_option = Some(*ctx.authority.key);
    let delegate_authority_option = Some(*ctx.authority.key);

    let mut metadata_data = ctx.metadata.try_borrow_mut_data()?;
    metadata_data[..8].copy_from_slice(&mint_authority_option.expect("Mint authority not found").to_bytes());
    metadata_data[8..].copy_from_slice(&delegate_authority_option.expect("Delegate authority not found").to_bytes());

    Ok(())
}

fn transfer_nft(ctx: TransferNFTContext) -> ProgramResult {
    // Verificar el contador y realizar la transferencia aquí
    Ok(())
}


