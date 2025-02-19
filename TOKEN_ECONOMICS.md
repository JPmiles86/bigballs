# BIGBALLS Token Economics Specification
Version: 0.1.2
Last Updated: Feb 17, 2025 10:24 AM
Created By: ARCH-2

## Token Fundamentals
- Name: "Big Balls"
- Symbol: "BIGBALLS"
- Decimals: 9
- Initial Supply: 1,000,000,000 (1 billion)
- Network: Solana
- Token Standard: SPL Token

## Initial Distribution
- Liquidity Pool: 95% (950,000,000 tokens)
- Development Wallet: 5% (50,000,000 tokens)

## Transaction Fees
Total Fee: 5% per transaction, distributed as:
- Reflection to Holders: 2%
- Marketing Wallet: 1.5%
- Auto-Burn: 1%
- Development: 0.5%

## Trading Limits

### Anti-Bot Measures
1. Launch Phase (First 24 hours)
   - Max Transaction: 0.1% of total supply (1,000,000 tokens)
   - Wallet Limit: 1% of total supply (10,000,000 tokens)
   - Cooldown: 60 seconds between transactions per wallet

2. Post-Launch Phase
   - Max Transaction: 1% of total supply (10,000,000 tokens)
   - Wallet Limit: 3% of total supply (30,000,000 tokens)
   - Cooldown: 30 seconds between transactions per wallet

### Trading Restrictions
1. Sell Limits
   - Maximum single sell: 1% of total supply
   - Daily sell limit per wallet: 2% of total supply
   - Cooldown between sells: 30 minutes

2. Buy Limits
   - Maximum single buy: 2% of total supply
   - Daily buy limit per wallet: 5% of total supply
   - Cooldown between buys: 5 minutes

## Reflection Mechanism
- 2% of each transaction redistributed to holders
- Distribution weighted by holder's token balance
- Minimum holding for reflection: 100,000 tokens
- Reflection calculation period: Real-time
- Excluded from reflection: Contract address, Burn address

## Marketing Wallet
- Initial allocation: 0 tokens
- Receives 1.5% of each transaction
- Maximum wallet size: 3% of total supply
- Spending requires multi-sig approval
- 24-hour timelock on large transfers (>0.5% of supply)

## Burn Mechanism
1. Automatic Burns
   - 1% of each transaction
   - Burns logged as events for visualization
   - No maximum burn limit

2. Manual Burns
   - Controlled by governance
   - Requires community vote
   - Minimum 24-hour notice
   - Maximum: 1% of supply per burn event

## Trading Enablement
1. Initial State
   - Trading disabled at launch
   - Transfer function limited to deployer

2. Launch Sequence
   - Deploy contract
   - Add initial liquidity
   - Enable trading with anti-bot measures
   - Transition to normal trading limits after 24 hours

## Visualization Parameters
Each token amount maps to ball properties:
- Size: log10(token_amount) * scale_factor
- Minimum visible amount: 1,000 tokens
- Maximum ball size: 3% of aquarium
- Merge threshold: Same wallet, multiple transactions within 10 seconds

## Emergency Controls
1. Circuit Breaker
   - Pause trading if price drops >50% in 1 hour
   - Require governance vote to resume
   - Maximum pause duration: 24 hours

2. Emergency Shutdown
   - Multi-sig requirement (3/5 signatures)
   - 24-hour timelock
   - Requires documented reason

## Governance Parameters
- Minimum holding for proposals: 0.1% of supply
- Voting period: 3 days
- Execution delay: 24 hours
- Quorum: 10% of total supply
- Majority threshold: 66%

## Technical Constraints
1. Transaction Processing
   - Maximum compute units: 200,000
   - Maximum accounts per tx: 32
   - Maximum instruction size: 1232 bytes

2. State Management
   - Account size limit: 10KB
   - Maximum PDAs: 256
   - Rent-exempt requirement: Yes

## Notes for Implementation
1. All percentages should use basis points (1% = 100bp)
2. Time measurements in Unix timestamps
3. Amount calculations must handle decimals correctly
4. All thresholds should be updatable via governance
5. Events must be emitted for all significant state changes
6. Include safety checks for arithmetic operations 