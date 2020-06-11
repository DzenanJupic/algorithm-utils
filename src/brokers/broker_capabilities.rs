#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BrokerCapability {
    MultipleDeposits,
    DepositBalances,
    DepositTransactions,

    OrderOverview,
    OrderChange,
    OrderDelete,

    BuyMarketOrder,
    BuyLimitOrder,
    BuyStopOrder,
    SellMarketOrder,
    SellLimitOrder,
    SellStopOrder,

    AllOrNoneOrder,
    ImmediateOrCancelOrder,
    FillOrKillOrder,

    PositionOverview,
    PositionChange,

    LongCallPosition,
    LongPutPosition,
    ShortCallPosition,
    ShortPutPosition,

    TrailingStopLoss,
    TakeProfit,
}