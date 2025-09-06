
# RWA Contract - Carbon Credits Management System 💰

A comprehensive Solana-based smart contract system for managing Real World Assets (RWA), specifically carbon credits, with integrated industry compliance tracking and Dutch auction mechanisms.

## Overview

This contract provides a complete ecosystem for carbon credit tokenization, industry onboarding with KYC verification, emissions tracking, and automated compliance monitoring. It enables transparent trading of carbon credits through Dutch auctions while maintaining regulatory compliance.

Read, the [blog](https://medium.com/@subhajitchaudhury05/dive-into-an-rwa-contract-with-token-extensions-418886da4203) by [Subh](https://github.com/subhdotsol) on RWA-Contract using Token-2022 extenion on this project.

## Deployment

**Network:** Solana Devnet  
**Program ID:** `4gBj3avgtDybWri9xiDQt7D3yaTiz3KbUysbVKq8Fcd4`  
**Deployment Signature:** `sDLswMNfHKSrrxDQs2uQjiLriPCf13bMC26iq5MXftZX58ZnPeadKSZrZruAhxPXYhSxrPw2CkjhiLHPVZyrqhQ`

## Architecture

The contract is modularized into five core components:

### 1. Access Control (`access_control.rs`)
- Role-based access control system
- Hierarchical permission management
- Admin role assignment and verification
- Granular action-based permissions

### 2. Carbon Credits (`carbon_credits.rs`)
- Token initialization with metadata (name, symbol, URI)
- CO2 tonnage tracking and project identification
- Expiry date management for time-bound credits
- Minting with strict authority controls

### 3. Industry Management (`industry.rs`)
- KYC verification and onboarding process
- Company registration with bond requirements
- Real-time emissions reporting
- Automated compliance status tracking

### 4. Dutch Auction System (`auction.rs`)
- Dynamic pricing mechanism (start price → end price over time)
- Real-time price calculation based on elapsed time
- Automated auction lifecycle management
- Bid placement with instant settlement

### 5. State Management (`state.rs`)
Defines all account structures:
- `Controller`: System admin management
- `UserRole`: Role-based permissions
- `CarbonToken`: Token metadata and supply tracking
- `Industry`: Company profiles and compliance data
- `DutchAuction`: Auction parameters and status

## Key Features

### Access Control System
- **Controller Account**: Manages system-wide admin privileges
- **Role Creation**: Define custom roles with specific action permissions
- **User Assignment**: Assign users to roles for granular access control
- **Permission Checking**: Runtime verification of user permissions

### Carbon Credit Tokenization
- **Metadata Rich**: Complete token information including CO2 tonnage, project ID, issuer details
- **Supply Tracking**: Real-time monitoring of total supply and circulation
- **Expiry Management**: Time-bound credits with automatic expiry handling
- **Authority Control**: Minting restricted to authorized parties only

### Industry Compliance
- **KYC Verification**: Mandatory verification process for industry onboarding
- **Bond System**: Financial commitment through bond requirements
- **Emissions Tracking**: Regular reporting of CO2 emissions
- **Compliance Monitoring**: Automatic status updates (Compliant/Non-Compliant/Frozen)

### Dutch Auction Trading
- **Dynamic Pricing**: Price decreases linearly from start to end price over time
- **Real-time Calculation**: Current price computed based on elapsed auction time
- **Instant Settlement**: Immediate bid processing and token allocation
- **Event Emission**: Complete auction lifecycle tracking through events

## Required Roles

The system defines three primary roles:

1. **MINT_AUTHORITY**: Can initialize carbon tokens and mint new credits
2. **KYC_AUTHORITY**: Can onboard and verify industries
3. **AUCTION_AUTHORITY**: Can create and manage Dutch auctions

## Workflow

### Industry Onboarding
1. KYC Authority verifies company credentials
2. Industry provides registration details and bond amount
3. System creates industry account with compliance tracking
4. Industry can begin emissions reporting

### Carbon Credit Lifecycle
1. Mint Authority initializes token with project metadata
2. Credits are minted to verified industries or traders
3. Industries use credits to offset reported emissions
4. Unused credits can be sold through Dutch auctions

### Auction Process
1. Auction Authority creates Dutch auction with price parameters
2. Price decreases linearly over auction duration
3. Bidders place bids at current market price
4. Tokens are automatically allocated upon successful bids
5. Auction closes when all tokens are sold or time expires

## Events & Monitoring

The contract emits comprehensive events for all major operations:

- `CarbonCreditsMinted`: Token creation and minting activities
- `IndustryOnboarded`: New company registrations
- `EmissionsReported`: CO2 emissions and compliance status updates
- `DutchAuctionCreated`: New auction announcements
- `BidPlaced`: Real-time bidding activity

## Security Features

- **Role-based Access Control**: All sensitive operations require appropriate permissions
- **Input Validation**: Comprehensive checks on all user inputs
- **Overflow Protection**: Safe arithmetic operations throughout
- **Account Verification**: PDA-based account derivation for security
- **Time-based Constraints**: Auction expiry and token validity periods

## Development Setup

### Prerequisites
- Rust 1.75+
- Solana CLI 1.18+
- Anchor Framework 0.31.1+
- Node.js 18+ with Yarn

### Build & Deploy
```bash
# Install dependencies
yarn install

# Build the program
anchor build

# Run tests
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

### Testing
The project includes comprehensive test coverage in `tests/rwa-contract.ts` using the Anchor testing framework.

## Usage Examples

### Initialize Access Control
```typescript
await program.methods
  .initializeAccessControl(adminPublicKey)
  .rpc();
```

### Create Carbon Token
```typescript
await program.methods
  .initializeCarbonToken(
    "Carbon Credit Token",
    "CCT",
    "https://metadata-uri.com",
    1000, // CO2 tonnes
    "PROJECT-001",
    expiryDate,
    "Green Energy Corp"
  )
  .rpc();
```

### Start Dutch Auction
```typescript
await program.methods
  .createDutchAuction(
    startPrice,
    endPrice,
    durationSeconds,
    tokensForSale
  )
  .rpc();
```

## ⚠️ Disclaimer

This contract is deployed on Solana Devnet for testing purposes. Always conduct thorough testing before mainnet deployment and consider professional security audits for production use.
