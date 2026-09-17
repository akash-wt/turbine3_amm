## Key Features

- **StableSwap Invariant**: Implements a hybrid model combining constant sum and constant product formulas to minimize slippage for pegged assets.
- **Dynamic Fee Mechanism**: Fees are not static; they scale based on the volatility of the asset ratio. As the pool diverges from the 1:1 peg, fees increase to protect liquidity providers and incentivize arbitrageurs.
- **Treasury Integration**: A dedicated treasury account automatically collects all swap fees, providing a revenue stream for the protocol.
- **Modular Architecture**: Built with a clean, maintainable structure separating state, constants, errors, and instruction logic.

## Technical Architecture

### Mathematical Model

The AMM utilizes the StableSwap invariant for two assets $x$ and $y$:
$$\text{Invariant} = x + y + \gamma xy$$
Where $\gamma$ (the amplification coefficient) determines the "flatness" of the curve. A larger $\gamma$ results in lower slippage near the 1:1 ratio.

**Dynamic Fees**:
The fee is calculated per swap:
$$\text{Fee} = \text{BaseFee} + \text{Multiplier} \times \left| \frac{x}{y} - 1 \right|$$

### Program Structure

- `lib.rs`: The entry point and instruction dispatcher.
- `state.rs`: Defines the `Pool` (reserves, invariant, LP supply) and `GlobalConfig` (treasury, fee parameters) accounts.
- `instructions/`:
  - `initialize_pool.rs`: Sets up the pool and global configuration.
  - `swap.rs`: Executes trades, calculates dynamic fees, and updates reserves.
  - `add_liquidity.rs`: Allows users to provide assets and mint LP tokens.
  - `remove_liquidity.rs`: Allows users to burn LP tokens and reclaim assets.
- `constants.rs`: Centralized program-wide constants.
- `error.rs`: Custom error codes for slippage, liquidity, and authorization.

## Testing

The program includes a comprehensive test suite using `LiteSVM` for fast, deterministic execution.

```bash
cargo test
```

### Test Coverage

- **Pool Initialization**: Verifies that the pool and global config are correctly created.
- **Swap Logic**: Validates the StableSwap output amounts and fee calculations.
- **Liquidity Management**: Ensures LP tokens are minted and burned correctly relative to reserves.
- **Dynamic Fees**: Verifies that fees increase as the asset ratio diverges from the peg.
