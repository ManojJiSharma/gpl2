//! General purpose GPL token utility functions

use arrayref::array_ref;
use gemachain_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};
use gpl_token::{
    instruction::set_authority,
    state::{Account, Mint},
};

use crate::{error::GovernanceError, tools::pack::unpack_coption_pubkey};

/// Creates and initializes GPL token account with PDA using the provided PDA seeds
#[allow(clippy::too_many_arguments)]
pub fn create_gpl_token_account_signed<'a>(
    payer_info: &AccountInfo<'a>,
    token_account_info: &AccountInfo<'a>,
    token_account_address_seeds: &[&[u8]],
    token_mint_info: &AccountInfo<'a>,
    token_account_owner_info: &AccountInfo<'a>,
    program_id: &Pubkey,
    system_info: &AccountInfo<'a>,
    gpl_token_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
    rent: &Rent,
) -> Result<(), ProgramError> {
    let create_account_instruction = system_instruction::create_account(
        payer_info.key,
        token_account_info.key,
        1.max(rent.minimum_balance(gpl_token::state::Account::get_packed_len())),
        gpl_token::state::Account::get_packed_len() as u64,
        &gpl_token::id(),
    );

    let (account_address, bump_seed) =
        Pubkey::find_program_address(token_account_address_seeds, program_id);

    if account_address != *token_account_info.key {
        msg!(
            "Create GPL Token Account with PDA: {:?} was requested while PDA: {:?} was expected",
            token_account_info.key,
            account_address
        );
        return Err(ProgramError::InvalidSeeds);
    }

    let mut signers_seeds = token_account_address_seeds.to_vec();
    let bump = &[bump_seed];
    signers_seeds.push(bump);

    invoke_signed(
        &create_account_instruction,
        &[
            payer_info.clone(),
            token_account_info.clone(),
            system_info.clone(),
        ],
        &[&signers_seeds[..]],
    )?;

    let initialize_account_instruction = gpl_token::instruction::initialize_account(
        &gpl_token::id(),
        token_account_info.key,
        token_mint_info.key,
        token_account_owner_info.key,
    )?;

    invoke(
        &initialize_account_instruction,
        &[
            payer_info.clone(),
            token_account_info.clone(),
            token_account_owner_info.clone(),
            token_mint_info.clone(),
            gpl_token_info.clone(),
            rent_sysvar_info.clone(),
        ],
    )?;

    Ok(())
}

/// Transfers GPL Tokens
pub fn transfer_gpl_tokens<'a>(
    source_info: &AccountInfo<'a>,
    destination_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    amount: u64,
    gpl_token_info: &AccountInfo<'a>,
) -> ProgramResult {
    let transfer_instruction = gpl_token::instruction::transfer(
        &gpl_token::id(),
        source_info.key,
        destination_info.key,
        authority_info.key,
        &[],
        amount,
    )
    .unwrap();

    invoke(
        &transfer_instruction,
        &[
            gpl_token_info.clone(),
            authority_info.clone(),
            source_info.clone(),
            destination_info.clone(),
        ],
    )?;

    Ok(())
}

/// Transfers GPL Tokens from a token account owned by the provided PDA authority with seeds
pub fn transfer_gpl_tokens_signed<'a>(
    source_info: &AccountInfo<'a>,
    destination_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    authority_seeds: &[&[u8]],
    program_id: &Pubkey,
    amount: u64,
    gpl_token_info: &AccountInfo<'a>,
) -> ProgramResult {
    let (authority_address, bump_seed) = Pubkey::find_program_address(authority_seeds, program_id);

    if authority_address != *authority_info.key {
        msg!(
                "Transfer GPL Token with Authority PDA: {:?} was requested while PDA: {:?} was expected",
                authority_info.key,
                authority_address
            );
        return Err(ProgramError::InvalidSeeds);
    }

    let transfer_instruction = gpl_token::instruction::transfer(
        &gpl_token::id(),
        source_info.key,
        destination_info.key,
        authority_info.key,
        &[],
        amount,
    )
    .unwrap();

    let mut signers_seeds = authority_seeds.to_vec();
    let bump = &[bump_seed];
    signers_seeds.push(bump);

    invoke_signed(
        &transfer_instruction,
        &[
            gpl_token_info.clone(),
            authority_info.clone(),
            source_info.clone(),
            destination_info.clone(),
        ],
        &[&signers_seeds[..]],
    )?;

    Ok(())
}

/// Asserts the given account_info represents a valid GPL Token account which is initialized and belongs to gpl_token program
pub fn assert_is_valid_gpl_token_account(account_info: &AccountInfo) -> Result<(), ProgramError> {
    if account_info.data_is_empty() {
        return Err(GovernanceError::GplTokenAccountDoesNotExist.into());
    }

    if account_info.owner != &gpl_token::id() {
        return Err(GovernanceError::GplTokenAccountWithInvalidOwner.into());
    }

    if account_info.data_len() != Account::LEN {
        return Err(GovernanceError::GplTokenInvalidTokenAccountData.into());
    }

    // TokeAccount layout:   mint(32), owner(32), amount(8), delegate(36), state(1), ...
    let data = account_info.try_borrow_data()?;
    let state = array_ref![data, 108, 1];

    if state == &[0] {
        return Err(GovernanceError::GplTokenAccountNotInitialized.into());
    }

    Ok(())
}

/// Asserts the given mint_info represents a valid GPL Token Mint account  which is initialized and belongs to gpl_token program
pub fn assert_is_valid_gpl_token_mint(mint_info: &AccountInfo) -> Result<(), ProgramError> {
    if mint_info.data_is_empty() {
        return Err(GovernanceError::GplTokenMintDoesNotExist.into());
    }

    if mint_info.owner != &gpl_token::id() {
        return Err(GovernanceError::GplTokenMintWithInvalidOwner.into());
    }

    if mint_info.data_len() != Mint::LEN {
        return Err(GovernanceError::GplTokenInvalidMintAccountData.into());
    }

    // In token program [36, 8, 1, is_initialized(1), 36] is the layout
    let data = mint_info.try_borrow_data().unwrap();
    let is_initialized = array_ref![data, 45, 1];

    if is_initialized == &[0] {
        return Err(GovernanceError::GplTokenMintNotInitialized.into());
    }

    Ok(())
}

/// Computationally cheap method to get mint from a token account
/// It reads mint without deserializing full account data
pub fn get_gpl_token_mint(token_account_info: &AccountInfo) -> Result<Pubkey, ProgramError> {
    assert_is_valid_gpl_token_account(token_account_info)?;

    // TokeAccount layout:   mint(32), owner(32), amount(8), ...
    let data = token_account_info.try_borrow_data()?;
    let mint_data = array_ref![data, 0, 32];
    Ok(Pubkey::new_from_array(*mint_data))
}

/// Computationally cheap method to get owner from a token account
/// It reads owner without deserializing full account data
pub fn get_gpl_token_owner(token_account_info: &AccountInfo) -> Result<Pubkey, ProgramError> {
    assert_is_valid_gpl_token_account(token_account_info)?;

    // TokeAccount layout:   mint(32), owner(32), amount(8)
    let data = token_account_info.try_borrow_data()?;
    let owner_data = array_ref![data, 32, 32];
    Ok(Pubkey::new_from_array(*owner_data))
}

/// Computationally cheap method to just get supply from a mint without unpacking the whole object
pub fn get_gpl_token_mint_supply(mint_info: &AccountInfo) -> Result<u64, ProgramError> {
    assert_is_valid_gpl_token_mint(mint_info)?;
    // In token program, 36, 8, 1, 1 is the layout, where the first 8 is supply u64.
    // so we start at 36.
    let data = mint_info.try_borrow_data().unwrap();
    let bytes = array_ref![data, 36, 8];

    Ok(u64::from_le_bytes(*bytes))
}

/// Computationally cheap method to just get authority from a mint without unpacking the whole object
pub fn get_gpl_token_mint_authority(
    mint_info: &AccountInfo,
) -> Result<COption<Pubkey>, ProgramError> {
    assert_is_valid_gpl_token_mint(mint_info)?;
    // In token program, 36, 8, 1, 1 is the layout, where the first 36 is authority.
    let data = mint_info.try_borrow_data().unwrap();
    let bytes = array_ref![data, 0, 36];

    unpack_coption_pubkey(bytes)
}

/// Asserts current mint authority matches the given authority and it's signer of the transaction
pub fn assert_gpl_token_mint_authority_is_signer(
    mint_info: &AccountInfo,
    mint_authority_info: &AccountInfo,
) -> Result<(), ProgramError> {
    let mint_authority = get_gpl_token_mint_authority(mint_info)?;

    if mint_authority.is_none() {
        return Err(GovernanceError::MintHasNoAuthority.into());
    }

    if !mint_authority.contains(mint_authority_info.key) {
        return Err(GovernanceError::InvalidMintAuthority.into());
    }

    if !mint_authority_info.is_signer {
        return Err(GovernanceError::MintAuthorityMustSign.into());
    }

    Ok(())
}

/// Sets new mint authority
pub fn set_gpl_token_mint_authority<'a>(
    mint_info: &AccountInfo<'a>,
    mint_authority: &AccountInfo<'a>,
    new_mint_authority: &Pubkey,
    gpl_token_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let set_authority_ix = set_authority(
        &gpl_token::id(),
        mint_info.key,
        Some(new_mint_authority),
        gpl_token::instruction::AuthorityType::MintTokens,
        mint_authority.key,
        &[],
    )?;

    invoke(
        &set_authority_ix,
        &[
            mint_info.clone(),
            mint_authority.clone(),
            gpl_token_info.clone(),
        ],
    )?;

    Ok(())
}

/// Asserts current token owner matches the given owner and it's signer of the transaction
pub fn assert_gpl_token_owner_is_signer(
    token_info: &AccountInfo,
    token_owner_info: &AccountInfo,
) -> Result<(), ProgramError> {
    let token_owner = get_gpl_token_owner(token_info)?;

    if token_owner != *token_owner_info.key {
        return Err(GovernanceError::InvalidTokenOwner.into());
    }

    if !token_owner_info.is_signer {
        return Err(GovernanceError::TokenOwnerMustSign.into());
    }

    Ok(())
}

/// Sets new token account owner
pub fn set_gpl_token_owner<'a>(
    token_info: &AccountInfo<'a>,
    token_owner: &AccountInfo<'a>,
    new_token_owner: &Pubkey,
    gpl_token_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let set_authority_ix = set_authority(
        &gpl_token::id(),
        token_info.key,
        Some(new_token_owner),
        gpl_token::instruction::AuthorityType::AccountOwner,
        token_owner.key,
        &[],
    )?;

    invoke(
        &set_authority_ix,
        &[
            token_info.clone(),
            token_owner.clone(),
            gpl_token_info.clone(),
        ],
    )?;

    Ok(())
}