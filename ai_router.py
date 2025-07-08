

def recommend_strategy(asset_type: str, value_usd: float) -> str:
    """
    Simple rule-based strategy router.
    Replace this with LLM-based decision engine later.
    """
    if asset_type.lower() == "real estate" and value_usd > 50000:
        return "Stake via CW20 + Auction Splitter"
    elif asset_type.lower() in ["gold", "commodity"]:
        return "Fixed Split Vault + CW721 Token"
    elif asset_type.lower() == "ip rights":
        return "Vesting Flow with Royalty Distributor"
    else:
        return "Standard CW20 Vault with Airdrop Rewards"
