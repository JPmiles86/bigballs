# Development Environment Setup

Before we can initialize the project, we need to install several prerequisites:

1. **Install Node.js**
```bash
# Using homebrew
brew install node@18
```

2. **Install pnpm**
```bash
# Using npm
npm install -g pnpm
```

3. **Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

4. **Install Solana**
```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
```

5. **Install Anchor**
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest
```

Once these are installed, we can proceed with project initialization. Would you like me to:

1. Verify these installations are complete?
2. Proceed with project initialization?
3. Create the Next.js frontend?

Please let me know once you've installed these prerequisites and we can continue with the setup. 