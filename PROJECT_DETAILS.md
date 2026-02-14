# Cornhole - Tier-2 Architecture Spec

## Overview
Cornhole is an on-chain, skill-based tossing game where users compete for $CORN prizes. Physics-based gameplay meets crypto economics — land bags on the board, sink holes, and climb the rankings in competitive 1v1 matches and tournament brackets.

## Market Opportunity
- Casual/skill games: $25B mobile gaming segment (2024)
- Esports/competitive casual growing 18% YoY
- Real cornhole has 30M+ players in US alone
- Target: Casual gamers, degen gamblers, esports fans, Web3 natives

## Core Mechanics

### 1. Gameplay
- **Physics-based throws**: Angle, power, wind affect trajectory
- **Scoring**: Board = 1 pt, Hole = 3 pts (cancellation scoring like real cornhole)
- **Match format**: First to 21 pts (must win by 2)
- **Controls**: Mobile-friendly drag/swipe, desktop click/drag

### 2. $CORN Token
- **Utility token** for all platform actions
- **Earn via**: Winning matches, tournaments, daily challenges
- **Spend on**: Entry fees, bag skins, board themes, tournament buy-ins
- **Staking**: Lock $CORN for yield + priority matchmaking

### 3. Match Modes
```
GAME MODES:
┌─────────────────────────────────────────────────┐
│  Casual       - Free play, no stakes            │
│  Ranked       - ELO-based, $CORN entry/rewards  │
│  High Roller  - 10x-100x entry, winner takes all│
│  Tournament   - Bracket play, prize pools       │
│  Practice     - AI opponent, skill training     │
└─────────────────────────────────────────────────┘
```

### 4. Anti-Cheat System
- **Commit-reveal throws**: Player commits hash of throw params → reveal after opponent's commit
- **Replay verification**: All throws stored on-chain, verifiable
- **Rating penalties**: Detected cheaters lose ELO + banned from ranked
- **Skill curve**: Natural variance prevents perfect bots

### 5. Customization NFTs
- **Bags**: 50+ designs (wood grain, pixel art, gold, animated)
- **Boards**: Themes (backyard, tailgate, crypto logos, seasonal)
- **Trails**: Particle effects when bags fly
- **Avatars**: Profile pics, frames, titles
- Rarity tiers: Common, Rare, Epic, Legendary, Mythic

### 6. Tournament System
- **Daily Tournaments**: Free entry, $CORN prizes
- **Weekly Majors**: Buy-in required, larger pools
- **Season Championships**: Top 64 qualify, massive prize pool
- **Private Leagues**: Create custom tournaments, invite friends
- Bracket formats: Single elim, double elim, round robin

## Tier System

| Tier | Requirement | Benefits |
|------|-------------|----------|
| **Rookie** | Join platform | Casual + Practice modes |
| **Tosser** | Win 10 matches | Ranked access, basic customization |
| **Ringer** | Reach 1500 ELO | High Roller access, 10% fee discount |
| **Champion** | Win tournament | 25% fee discount, exclusive skins |
| **Legend** | Top 100 seasonal | Revenue share, custom board design |

## Technical Architecture

### Smart Contracts (Solidity/EVM)
```
CornToken.sol          - ERC-20 utility token
CornholeMatch.sol      - Match state, commit-reveal logic
CornholeTournament.sol - Bracket management, prize distribution
CornholeNFT.sol        - ERC-1155 cosmetics
CornholeStaking.sol    - $CORN staking vault
VRFConsumer.sol        - Chainlink VRF for wind randomness
```

### Game Engine
- Phaser.js for 2D physics (or Three.js for 3D option)
- Matter.js physics engine
- Client-side prediction, server reconciliation
- Deterministic physics replay for verification

### Frontend Stack
- Next.js 14 + TypeScript
- RainbowKit + wagmi for wallet
- Socket.io for real-time multiplayer
- Framer Motion for UI animations

### Backend Services
- Node.js game server (matchmaking, state sync)
- Redis for match state, queues
- PostgreSQL for user profiles, history
- The Graph for on-chain indexing

## Revenue Model
1. **Match fees**: 5% rake on ranked/high-roller entry
2. **Tournament fees**: 10% of prize pool
3. **NFT sales**: Primary mints + 5% secondary royalties
4. **$CORN trading**: 1% buy/sell tax
5. **Sponsorships**: Branded boards/events

## Anti-Exploitation
- **Rate limiting**: Max matches per hour
- **Skill-based matchmaking**: ±200 ELO range
- **Minimum stake**: Prevents micro-grinding
- **Cooldown timers**: Between high-roller matches

## Go-to-Market
1. **Phase 1**: Closed beta, core gameplay, invite-only
2. **Phase 2**: Public launch, ranked mode, first tournament
3. **Phase 3**: NFT drop, staking, seasonal system
4. **Phase 4**: Mobile app, cross-chain expansion

## Success Metrics
- DAU/MAU ratio > 30%
- Match completion rate > 90%
- Tournament fill rate > 80%
- $CORN velocity (daily volume / supply) > 5%
