# BigBalls ($BIGBALLS)

Interactive 3D visualization of $BIGBALLS token holders, where each holder's balance is represented by a physics-enabled ball in a virtual aquarium.

## Project Overview

BigBalls is a unique Solana token that combines:
- Meme coin economics
- Interactive 3D visualization
- Real-time holder tracking
- Physics-based animations
- Community engagement features

## Quick Start

### Prerequisites
- Node.js >=18.0.0
- pnpm
- Rust
- Solana CLI
- Anchor Framework

### Development Setup
```bash
# Clone the repository
git clone [repository-url]
cd bigballs

# Install dependencies
pnpm install

# Setup Solana
solana config set --url devnet

# Build Anchor program
cd programs/bigballs
anchor build

# Start frontend development
cd ../../app
pnpm dev
```

## Project Structure
- `/programs` - Solana smart contracts
- `/app` - Next.js frontend application
- `/tests` - Test suites
- `/scripts` - Deployment and utility scripts

## Features
- 3D ball visualization
- Real-time wallet tracking
- Physics-based animations
- Interactive UI
- Background music system

## Development
See [PROJECT_STRUCTURE.md](./PROJECT_STRUCTURE.md) for detailed development setup.

## Testing
```bash
# Run contract tests
anchor test

# Run frontend tests
cd app
pnpm test
```

## Deployment
Deployment instructions will be added after initial development phase.

## License
MIT

## Version
0.1.2 