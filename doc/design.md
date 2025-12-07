# DefPool Design Document

## 1. Overview
DefPool is a profit-switching crypto mining pool designed to maximize miner revenue by automatically switching hashrate to the most profitable coin. The system converts all earnings into Bitcoin (BTC) and distributes them to miners.

## 2. Architecture
The system consists of three main components:

### 2.1. DefPool Proxy (`defpool-proxy`)
- **Role**: Acts as the gateway for miners.
- **Protocol**: Implements Stratum V2 for efficient and secure communication.
- **Functionality**:
    - Accepts connections from mining hardware.
    - Receives switching commands from the `defpool-server`.
    - Transparently redirects hashrate to the target coin's daemon/pool without disconnecting the miner (where supported) or manages the reconnection process.

### 2.2. DefPool Server (`defpool-server`)
- **Role**: The central brain of the operation.
- **Functionality**:
    - **Profitability Monitoring**: Continuously checks market data (exchange rates, difficulty, block rewards) for supported coins.
    - **Configuration**: Manages a whitelist of supported coins defined by the Administrator.
    - **Decision Engine**: Calculates the most profitable coin to mine from the whitelist.
    - **Command & Control**: Sends switching commands to connected `defpool-proxy` instances.
    - **Accounting**: Tracks shares submitted by miners for each coin.
    - **API Layer**: Exposes REST/GraphQL endpoints for the `defpool-portal` to fetch stats and configuration.
    - **Real-time Publisher**: Pushes live updates (hashrate, earnings, active coin) to clients via WebSockets or SSE.

### 2.3. Miner
- **Role**: The end-user mining hardware/software.
- **Connection**: Connects to `defpool-proxy` via Stratum V2.

### 2.4. DefPool Portal (`defpool-portal`)
- **Role**: Client-facing web application.
- **Functionality**:
    - **Dashboard**: Displays real-time mining statistics (hashrate, earnings, current coin).
    - **Configuration**: Allows users to manage payout addresses and notification settings.
    - **Live Updates**: Receives real-time data from the Server via WebSockets.

## 3. Core Logic

### 3.1. Profit Switching
1.  **Data Collection**: The Server aggregates real-time data for all **whitelisted coins**:
    - Coin Price (in BTC).
    - Network Difficulty.
    - Block Reward.
2.  **Calculation**: `Profitability = (Block Reward * Coin Price) / Difficulty`.
3.  **Thresholding**: To prevent rapid thrashing, a hysteresis threshold (e.g., new coin must be X% more profitable for Y minutes) is applied.
4.  **Switching**:
    - Server identifies the new best coin.
    - Server broadcasts a "Switch" command to Proxies.
    - Proxies update the upstream connection to the new coin's network.

### 3.2. Payouts (Auto-Exchange)
1.  **Mining**: Miners earn rewards in various Altcoins (e.g., LTC, RVN, etc.).
2.  **Conversion**: The system automatically exchanges mined Altcoins for Bitcoin (BTC) on integrated exchanges.
3.  **Distribution**:
    - Miners' balances are denominated in BTC.
    - Payouts are triggered when the BTC balance exceeds a user-defined threshold.

## 4. Technical Stack
- **Language**: Rust (suggested for performance and safety) or Go.
- **Frontend**: React (for `defpool-portal`).
- **Communication**:
    - Stratum V2 (Miner <-> Proxy).
    - REST/GraphQL + WebSockets (Portal <-> Server).
- **Database**: Time-series DB for stats, Relational DB for user accounts/accounting.

### 4. Deployment Models

DefPool supports two modes of operation, which can be mixed:

#### A. Pool Mining (External Upstream)
The Proxy forwards hashrate to an existing external pool (e.g., SupportXMR, AntPool).
- **Flow**: `Miner` -> `DefPool Proxy` -> `External Pool`
- **Pros**: Simple, consistent payouts from the external pool.
- **Cons**: Pool fees, less control.

#### B. Solo Mining (Internal Upstream)
The Proxy forwards hashrate to a local "Coin Adapter" or "Bridge" that connects directly to the coin's daemon.
- **Flow**: `Miner` -> `DefPool Proxy` -> `Coin Adapter (SV2 Pool)` -> `Coin Daemon`
- **Components**:
    - **Coin Daemon**: The actual blockchain node (e.g., `bitcoind`, `monerod`).
    - **Coin Adapter**: A lightweight Stratum V2 pool implementation (e.g., `pool_sv2`) that acts as a bridge. It converts SV2 messages into RPC calls (`getblocktemplate`) for the daemon.
- **Pros**: No pool fees, full block rewards.
- **Cons**: High variance (luck), requires running full nodes.

## 5. Component Interactionsts & Compatibility

### 5.1. Same-Algorithm Switching
- **Constraint**: Most standard mining hardware (ASICs) and software (e.g., CGMiner, BMMiner) are designed to mine a specific algorithm (e.g., SHA-256, Scrypt) and cannot switch algorithms dynamically without a restart or reconfiguration.
- **Design Decision**: DefPool focuses on **Same-Algorithm Switching**.
    - *Example*: A miner connected to a SHA-256 port can be switched between Bitcoin (BTC), Bitcoin Cash (BCH), and other SHA-256 coins.
    - *Limitation*: A miner cannot be switched from a SHA-256 coin to a Scrypt coin.
- **Future Scope**: Multi-algorithm switching would require a custom miner agent or specialized management software, which is out of scope for the initial version.
