
pub enum EscrowInstruction {

    /// Starts the trade by creating and populating an escrow account
    /// and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction
    ///     and owned by the initializer (token X, amount sent)
    /// 2. `[]` The initializer's token account for the token Y they will receive (should the trade
    ///     go through)
    /// 3. `[writable]` The escrow account. It will hold all necessary info about the trade.
    /// 4. `[]` The token program
    InitEscrow {
        /// The amount party A excepts to receive of token Y.
        amount: u64
    }
}