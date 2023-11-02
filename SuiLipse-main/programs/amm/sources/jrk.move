module sui_lipse::jrk{
    use sui::coin;
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    /// The type identifier of coin. The coin will have a type
    /// tag of kind: `Coin<package_object::mycoin::MYCOIN>`
    /// Make sure that the name of the type matches the module's name.
    struct JRK has drop {}



    /// Module initializer is called once on module publish. A treasury
    /// cap is sent to the publisher, who then controls minting and burning
    fun init(witness: JRK, ctx: &mut TxContext) {
        transfer::transfer(
            coin::create_currency(witness, 10, ctx),
            tx_context::sender(ctx)
        )
    }
}
