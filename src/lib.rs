// use borsh::{BorshDeserialize, BorshSerialize}; 
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    // msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    // sysvar::{rent::Rent, Sysvar},
    system_instruction,
    program::{invoke},
};

use spl_token::{
    instruction::*,

};

use solana_sdk::{signature::Keypair, signer::Signer};
use std::str::FromStr;



entrypoint!(process_instructions);

pub enum Instructions{
    CreateAccount {
        // cost: u64
    },
}

impl Instructions{
    fn unpackinst(input: &[u8]) -> Result<Self, ProgramError>{
        let (&instr, _) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match instr{
            0 => {
                // let (&cost, _) = rest.split_first().ok_or(ProgramError::InvalidInstructionData)?;

                Self::CreateAccount{
                    // cost
                }
    
            }

            _ => return Err(ProgramError::InvalidInstructionData.into())
        })
    }
}



pub fn process_instructions(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8])-> ProgramResult{
    let instruction = Instructions::unpackinst(instruction_data)?;

    let account_info_iter =  &mut accounts.iter();

    match instruction {
        Instructions::CreateAccount{
            // cost
        } => {
            let payer_account_info = next_account_info(account_info_iter)?;
            let vault = next_account_info(account_info_iter)?;
            let temp_key = Pubkey::from_str("G473EkeR5gowVn8CRwTSDop3zPwaNixwp62qi7nyVf4z").unwrap();
            if vault.key != &temp_key {
                Err(ProgramError::InvalidInstructionData)?
            }

            // let program_id = next_account_info(account_info_iter)?;
            // let space:usize = 1000;
            // let rent_lamports = Rent::get()?.minimum_balance(space);
            let price: u64 = (0.5 * (10^9) as f64) as u64;

            invoke(
                &system_instruction::transfer(
                    &payer_account_info.key,
                    &temp_key,
                    price,
                ),
                &[
                    payer_account_info.clone(),
                    vault.clone()
                ]
            )?;

            let mint_pubkey = Keypair::new();
            let keypair_pubkey = Signer::pubkey(&mint_pubkey);

            let mint_authority_pubkey = program_id.clone();
            let freeze_authority_pubkey = program_id.clone();
            let decimals = 0;
            let token_program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
            
            invoke(
                &initialize_mint(&token_program_id, &keypair_pubkey, &mint_authority_pubkey, Some(&freeze_authority_pubkey), decimals)?,
                &[]
            )?;




        }
    }
    

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(5*5, 5*5);
    }
}