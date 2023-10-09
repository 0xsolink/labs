use anchor_lang::prelude::*;

declare_id!("HpSRXbwyB1wtiZGhWg1ce6yDXjgyZ523cQ5PyXMEm7zw");

#[program]
pub mod lssvm_pair {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn swapTokenForAnyNFTs(){

    }
    pub fn swapNFTsForToken(){

    }

    pub fn getBuyNFTQuote(){

    }

    pub fn getSellNFTQuote(){

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



}

#[derive(Accounts)]
pub struct Initialize {}
