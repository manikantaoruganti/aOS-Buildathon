# aOS-Buildathon
# 📁 OptiVaultAI – Project Directory Structure (Andromeda Hackathon)

## 🔗 Root Structure
```
/optivaultai
├── contracts/                # Rust CosmWasm contracts (CW721, Splitter, etc.)
│   ├── Cargo.toml
│   └── src/
├── ado_configs/              # ADO deployment configs (.ado.json)
├── backend/                  # FastAPI server (API gateway + AI router)
│   ├── main.py
│   └── ai_router.py
├── frontend/                 # React + Tailwind App
│   ├── src/
│   │   ├── App.jsx
│   │   └── index.js
│   └── public/
├── ai/                       # AI scripts (OpenAI, ML routing)
│   └── yield_strategy.py
├── README.md                 # Project documentation
├── LICENSE                   # Open source license
└── .github/workflows/        # CI/CD automation
    └── deploy.yml


