
from fastapi import FastAPI
from pydantic import BaseModel
from ai_router import recommend_strategy
import uuid

app = FastAPI()

class TokenizeAssetRequest(BaseModel):
    asset_name: str
    asset_type: str
    value_usd: float
    owner_address: str

@app.post("/tokenize")
def tokenize_asset(req: TokenizeAssetRequest):
    # Simulate token creation with UUID
    token_id = str(uuid.uuid4())
    return {
        "status": "success",
        "token_id": token_id,
        "asset": req
    }

@app.get("/route-defi")
def route_defi(asset_type: str, value_usd: float):
    strategy = recommend_strategy(asset_type, value_usd)
    return {
        "strategy": strategy,
        "asset_type": asset_type,
        "value_usd": value_usd
    }
