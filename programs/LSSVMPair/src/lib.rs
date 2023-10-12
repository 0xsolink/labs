use anchor_lang::prelude::*;

declare_id!("HpSRXbwyB1wtiZGhWg1ce6yDXjgyZ523cQ5PyXMEm7zw");

#[program]
pub mod lssvm_pair {
    use super::*;

    pub fn initialize(ctx: Context<SwapInstruction>) -> Result<()> {
        Ok(())
    }
/*
    pub fn swapTokenForAnyNFTs(){

    }
    pub fn swapNFTsForToken(){

    }

    pub fn bondingCurve(){

    }
    pub fn _sendAnyNFTsToRecipient(){

    }
    pub fn _takeNFTsFromSender(){

    }
    pub fn _pullTokenInputAndPayProtocolFee(){

    }
    
    pub fn _withdrawToken(){

    }
    pub fn _sendTokenOutput(){

    }
*/



}

#[account]
pub struct Initialize {
    /// all swap fees
    pub fees: Fees,
    /// swap curve info for pool, including CurveType and anything
    /// else that may be required
    pub swap_curve: SwapCurve,
}


#[account]
pub struct Swap {
    /// SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub amount_in: u64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub minimum_amount_out: u64,
}


#[derive(Accounts)]
pub enum SwapInstruction{
    Initialize(Initialize),
    Swap(Swap)

}


impl SwapInstruction {
    /// Unpacks a byte buffer into a [SwapInstruction](enum.SwapInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(SwapError::InvalidInstruction)?;
        Ok(match tag {
            0 => {
                if rest.len() >= Fees::LEN {
                    let (fees, rest) = rest.split_at(Fees::LEN);
                    let fees = Fees::unpack_unchecked(fees)?;
                    let swap_curve = SwapCurve::unpack_unchecked(rest)?;
                    Self::Initialize(Initialize { fees, swap_curve })
                } else {
                    return Err(SwapError::InvalidInstruction.into());
                }
            }
            1 => {
                let (amount_in, rest) = Self::unpack_u64(rest)?;
                let (minimum_amount_out, _rest) = Self::unpack_u64(rest)?;
                Self::Swap(Swap {
                    amount_in,
                    minimum_amount_out,
                })
            }
            2 => {
                let (pool_token_amount, rest) = Self::unpack_u64(rest)?;
                let (maximum_token_a_amount, rest) = Self::unpack_u64(rest)?;
                let (maximum_token_b_amount, _rest) = Self::unpack_u64(rest)?;
                Self::DepositAllTokenTypes(DepositAllTokenTypes {
                    pool_token_amount,
                    maximum_token_a_amount,
                    maximum_token_b_amount,
                })
            }
            3 => {
                let (pool_token_amount, rest) = Self::unpack_u64(rest)?;
                let (minimum_token_a_amount, rest) = Self::unpack_u64(rest)?;
                let (minimum_token_b_amount, _rest) = Self::unpack_u64(rest)?;
                Self::WithdrawAllTokenTypes(WithdrawAllTokenTypes {
                    pool_token_amount,
                    minimum_token_a_amount,
                    minimum_token_b_amount,
                })
            }
            4 => {
                let (source_token_amount, rest) = Self::unpack_u64(rest)?;
                let (minimum_pool_token_amount, _rest) = Self::unpack_u64(rest)?;
                Self::DepositSingleTokenTypeExactAmountIn(DepositSingleTokenTypeExactAmountIn {
                    source_token_amount,
                    minimum_pool_token_amount,
                })
            }
            5 => {
                let (destination_token_amount, rest) = Self::unpack_u64(rest)?;
                let (maximum_pool_token_amount, _rest) = Self::unpack_u64(rest)?;
                Self::WithdrawSingleTokenTypeExactAmountOut(WithdrawSingleTokenTypeExactAmountOut {
                    destination_token_amount,
                    maximum_pool_token_amount,
                })
            }
            _ => return Err(SwapError::InvalidInstruction.into()),
        })
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(SwapError::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(SwapError::InvalidInstruction.into())
        }
    }

    /// Packs a [SwapInstruction](enum.SwapInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            Self::Initialize(Initialize { fees, swap_curve }) => {
                buf.push(0);
                let mut fees_slice = [0u8; Fees::LEN];
                Pack::pack_into_slice(fees, &mut fees_slice[..]);
                buf.extend_from_slice(&fees_slice);
                let mut swap_curve_slice = [0u8; SwapCurve::LEN];
                Pack::pack_into_slice(swap_curve, &mut swap_curve_slice[..]);
                buf.extend_from_slice(&swap_curve_slice);
            }
            Self::Swap(Swap {
                amount_in,
                minimum_amount_out,
            }) => {
                buf.push(1);
                buf.extend_from_slice(&amount_in.to_le_bytes());
                buf.extend_from_slice(&minimum_amount_out.to_le_bytes());
            }
            Self::DepositAllTokenTypes(DepositAllTokenTypes {
                pool_token_amount,
                maximum_token_a_amount,
                maximum_token_b_amount,
            }) => {
                buf.push(2);
                buf.extend_from_slice(&pool_token_amount.to_le_bytes());
                buf.extend_from_slice(&maximum_token_a_amount.to_le_bytes());
                buf.extend_from_slice(&maximum_token_b_amount.to_le_bytes());
            }
            Self::WithdrawAllTokenTypes(WithdrawAllTokenTypes {
                pool_token_amount,
                minimum_token_a_amount,
                minimum_token_b_amount,
            }) => {
                buf.push(3);
                buf.extend_from_slice(&pool_token_amount.to_le_bytes());
                buf.extend_from_slice(&minimum_token_a_amount.to_le_bytes());
                buf.extend_from_slice(&minimum_token_b_amount.to_le_bytes());
            }
            Self::DepositSingleTokenTypeExactAmountIn(DepositSingleTokenTypeExactAmountIn {
                source_token_amount,
                minimum_pool_token_amount,
            }) => {
                buf.push(4);
                buf.extend_from_slice(&source_token_amount.to_le_bytes());
                buf.extend_from_slice(&minimum_pool_token_amount.to_le_bytes());
            }
            Self::WithdrawSingleTokenTypeExactAmountOut(
                WithdrawSingleTokenTypeExactAmountOut {
                    destination_token_amount,
                    maximum_pool_token_amount,
                },
            ) => {
                buf.push(5);
                buf.extend_from_slice(&destination_token_amount.to_le_bytes());
                buf.extend_from_slice(&maximum_pool_token_amount.to_le_bytes());
            }
        }
        buf
    }
}

pub fn swap(
    program_id: &Pubkey,
    source_token_program_id: &Pubkey,
    destination_token_program_id: &Pubkey,
    pool_token_program_id: &Pubkey,
    swap_pubkey: &Pubkey,
    authority_pubkey: &Pubkey,
    user_transfer_authority_pubkey: &Pubkey,
    source_pubkey: &Pubkey,
    swap_source_pubkey: &Pubkey,
    swap_destination_pubkey: &Pubkey,
    destination_pubkey: &Pubkey,
    pool_mint_pubkey: &Pubkey,
    pool_fee_pubkey: &Pubkey,
    source_mint_pubkey: &Pubkey,
    destination_mint_pubkey: &Pubkey,
    host_fee_pubkey: Option<&Pubkey>,
    instruction: Swap,
) -> Result<Instruction, ProgramError> {
    let data = SwapInstruction::Swap(instruction).pack();

    let mut accounts = vec![
        AccountMeta::new_readonly(*swap_pubkey, false),
        AccountMeta::new_readonly(*authority_pubkey, false),
        AccountMeta::new_readonly(*user_transfer_authority_pubkey, true),
        AccountMeta::new(*source_pubkey, false),
        AccountMeta::new(*swap_source_pubkey, false),
        AccountMeta::new(*swap_destination_pubkey, false),
        AccountMeta::new(*destination_pubkey, false),
        AccountMeta::new(*pool_mint_pubkey, false),
        AccountMeta::new(*pool_fee_pubkey, false),
        AccountMeta::new_readonly(*source_mint_pubkey, false),
        AccountMeta::new_readonly(*destination_mint_pubkey, false),
        AccountMeta::new_readonly(*source_token_program_id, false),
        AccountMeta::new_readonly(*destination_token_program_id, false),
        AccountMeta::new_readonly(*pool_token_program_id, false),
    ];
    if let Some(host_fee_pubkey) = host_fee_pubkey {
        accounts.push(AccountMeta::new(*host_fee_pubkey, false));
    }

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn initialize(
    program_id: &Pubkey,
    token_program_id: &Pubkey,
    swap_pubkey: &Pubkey,
    authority_pubkey: &Pubkey,
    token_a_pubkey: &Pubkey,
    token_b_pubkey: &Pubkey,
    pool_pubkey: &Pubkey,
    fee_pubkey: &Pubkey,
    destination_pubkey: &Pubkey,
    fees: Fees,
    swap_curve: SwapCurve,
) -> Result<Instruction, ProgramError> {
    let init_data = SwapInstruction::Initialize(Initialize { fees, swap_curve });
    let data = init_data.pack();

    let accounts = vec![
        AccountMeta::new(*swap_pubkey, true),
        AccountMeta::new_readonly(*authority_pubkey, false),
        AccountMeta::new_readonly(*token_a_pubkey, false),
        AccountMeta::new_readonly(*token_b_pubkey, false),
        AccountMeta::new(*pool_pubkey, false),
        AccountMeta::new_readonly(*fee_pubkey, false),
        AccountMeta::new(*destination_pubkey, false),
        AccountMeta::new_readonly(*token_program_id, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

#[test]
    fn pack_intialize() {
        let trade_fee_numerator: u64 = 1;
        let trade_fee_denominator: u64 = 4;
        let owner_trade_fee_numerator: u64 = 2;
        let owner_trade_fee_denominator: u64 = 5;
        let owner_withdraw_fee_numerator: u64 = 1;
        let owner_withdraw_fee_denominator: u64 = 3;
        let host_fee_numerator: u64 = 5;
        let host_fee_denominator: u64 = 20;
        let fees = Fees {
            trade_fee_numerator,
            trade_fee_denominator,
            owner_trade_fee_numerator,
            owner_trade_fee_denominator,
            owner_withdraw_fee_numerator,
            owner_withdraw_fee_denominator,
            host_fee_numerator,
            host_fee_denominator,
        };
        let token_b_offset: u64 = 1_000_000_000;
        let curve_type = CurveType::Offset;
        let calculator = Arc::new(OffsetCurve { token_b_offset });
        let swap_curve = SwapCurve {
            curve_type,
            calculator,
        };
        let check = SwapInstruction::Initialize(Initialize { fees, swap_curve });
        let packed = check.pack();
        let mut expect = vec![0u8];
        expect.extend_from_slice(&trade_fee_numerator.to_le_bytes());
        expect.extend_from_slice(&trade_fee_denominator.to_le_bytes());
        expect.extend_from_slice(&owner_trade_fee_numerator.to_le_bytes());
        expect.extend_from_slice(&owner_trade_fee_denominator.to_le_bytes());
        expect.extend_from_slice(&owner_withdraw_fee_numerator.to_le_bytes());
        expect.extend_from_slice(&owner_withdraw_fee_denominator.to_le_bytes());
        expect.extend_from_slice(&host_fee_numerator.to_le_bytes());
        expect.extend_from_slice(&host_fee_denominator.to_le_bytes());
        expect.push(curve_type as u8);
        expect.extend_from_slice(&token_b_offset.to_le_bytes());
        expect.extend_from_slice(&[0u8; 24]);
        assert_eq!(packed, expect);
        let unpacked = SwapInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);
    }


    #[test]
    fn pack_swap() {
        let amount_in: u64 = 2;
        let minimum_amount_out: u64 = 10;
        let check = SwapInstruction::Swap(Swap {
            amount_in,
            minimum_amount_out,
        });
        let packed = check.pack();
        let mut expect = vec![1];
        expect.extend_from_slice(&amount_in.to_le_bytes());
        expect.extend_from_slice(&minimum_amount_out.to_le_bytes());
        assert_eq!(packed, expect);
        let unpacked = SwapInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);
    }
