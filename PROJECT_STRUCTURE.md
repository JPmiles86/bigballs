# BigBalls Project Structure
Version: 0.1.2
Last Updated: Feb 17, 2025 10:24 AM
Updated By: ARCH-2

## Repository Structure
```
bigballs/
├── .cursorrules                 # Cursor IDE configuration
├── README.md                    # Project documentation
├── TASK_SPECS.md               # Task specifications
├── package.json                # Root package configuration
├── pnpm-lock.yaml             # Lock file
├── programs/                   # Solana programs (smart contracts)
│   └── bigballs/
│       ├── Cargo.toml          # Rust dependencies
│       └── src/
│           └── lib.rs          # Token contract implementation
├── app/                        # Next.js frontend application
│   ├── package.json           # Frontend dependencies
│   ├── next.config.js         # Next.js configuration
│   ├── tsconfig.json          # TypeScript configuration
│   ├── public/                # Static assets
│   │   ├── audio/            # Sound effects and music
│   │   └── models/           # 3D models and textures
│   └── src/
│       ├── components/        # React components
│       │   ├── Aquarium/     # 3D environment
│       │   ├── Balls/        # Ball visualization
│       │   ├── UI/           # User interface
│       │   └── Metrics/      # Market metrics display
│       ├── hooks/            # Custom React hooks
│       ├── services/         # Backend services
│       │   ├── wallet/       # Wallet integration
│       │   ├── physics/      # Physics engine
│       │   └── blockchain/   # Solana interaction
│       ├── types/            # TypeScript definitions
│       ├── utils/            # Utility functions
│       └── pages/            # Next.js pages
├── tests/                     # Test suites
│   ├── contract/             # Smart contract tests
│   └── frontend/             # Frontend tests
└── scripts/                   # Deployment and utility scripts
```

## Initial Setup Steps

1. **Repository Initialization**
```bash
# Create project directory
mkdir bigballs
cd bigballs

# Initialize package manager
pnpm init

# Create necessary directories
mkdir -p programs/bigballs/src
mkdir -p app/src/{components,hooks,services,types,utils}
mkdir -p app/public/{audio,models}
mkdir -p tests/{contract,frontend}
```

2. **Solana Program Setup**
```bash
# Initialize Anchor project
anchor init programs/bigballs

# Configure Solana toolchain
rustup default stable
rustup component add rustfmt
```

3. **Frontend Setup**
```bash
# Create Next.js app
cd app
pnpm create next-app . --typescript

# Install dependencies
pnpm add three @react-three/fiber @react-three/drei
pnpm add @solana/web3.js @solana/wallet-adapter-react
pnpm add @project-serum/anchor
```

## Development Environment

### Required Tools
- Node.js >=18.0.0
- pnpm
- Rust
- Solana CLI
- Anchor Framework
- Git

### IDE Configuration
- Use Cursor IDE
- Follow `.cursorrules` configuration
- Enable TypeScript strict mode
- Configure Prettier and ESLint

## Next Steps

1. **For DEV Agent**
   - Focus on Solana program implementation
   - Start with basic SPL token functionality
   - Add security features progressively
   - Write comprehensive tests

2. **For VIS Agent** (after DEV completion)
   - Initialize Next.js frontend
   - Set up Three.js environment
   - Implement basic ball physics
   - Create visualization components

## Notes
- All development must follow TypeScript strict mode
- Use Rust 2021 edition for Solana programs
- Follow Solana program security best practices
- Implement proper error handling
- Document all code thoroughly 