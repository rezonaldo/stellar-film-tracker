# Stellar Film Tracker DApp

**Stellar Film Tracker DApp** - Blockchain-Based Decentralized Movie Watchlist

## Project Description

Stellar Film Tracker DApp is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure, immutable platform for managing your personal movie and TV show watchlist directly on the blockchain. The contract ensures that your entertainment data is stored transparently and is only manageable through predefined smart contract functions, eliminating reliance on centralized tracking applications like Letterboxd or IMDb.

The system allows users to add films (with titles and genres), view their entire watchlist, and remove films once they have watched them, leveraging the efficiency and security of the Stellar network. Each film entry is uniquely identified and stored within the contract's instance storage, ensuring data persistence and reliability.

## Project Vision

Our vision is to revolutionize how cinephiles and casual viewers track their entertainment journey by:

- **Decentralizing Data**: Moving viewing histories and watchlists from centralized servers to a global, distributed blockchain.
- **Ensuring Ownership**: Empowering users to have complete control and ownership over their personal entertainment data.
- **Guaranteeing Immutability**: Providing a permanent, tamper-proof record of movie lists that cannot be altered or monetized by third parties without consent.
- **Enhancing Privacy**: Leveraging blockchain security to protect personal viewing preferences.
- **Building Trustless Systems**: Creating a platform where data integrity is guaranteed by code, paving the way for future Web3 entertainment integrations.

## Key Features

### 1. **Simple Film Addition**
- Add movies to your watchlist with just one function call.
- Specify the `title` and `genre` for each film.
- Automated PRNG-based ID generation for unique identification.
- Persistent storage on the Stellar blockchain.

### 2. **Efficient Data Retrieval**
- Fetch all stored films in a single call.
- Structured data representation for easy frontend UI integration.
- Quick access to your entire watchlist collection.

### 3. **Secure Deletion**
- Remove specific films from your watchlist using their unique IDs (e.g., after watching them).
- Permanent removal from the contract storage for efficient space management.
- Immediate update of the watchlist array after deletion.

### 4. **Transparency and Security**
- View all watchlist activities on the blockchain.
- Immutable records of film addition and deletion.
- Protected against unauthorized modifications.

### 5. **Stellar Network Integration**
- Leverages the high speed and ultra-low cost of the Stellar network.
- Built using the modern Rust-based Soroban Smart Contract SDK.
- Scalable architecture for growing lists of movies.

## Contract Details

- **Contract Address (Testnet):** CCTKC2FF3JNTAXYDVI7ZHYMLGYJHT6QTDPVJGH3AXVXZLQVW5O2X3RT6
*(Don't forget to replace this placeholder with your actual deployed Contract ID)*

## Future Scope

### Short-Term Enhancements
1. **Rating System**: Allow users to add a 1-5 star rating when removing a film from the watchlist to an "already watched" list.
2. **Review Storage**: Add support for short text reviews attached to specific films.
3. **Advanced Filtering**: Implement search and filter functionalities by genre.

### Medium-Term Development
4. **Collaborative Watchlists**: Implement multi-signature requirements for shared group watchlists (e.g., movie clubs, couples).
5. **Recommendation Engine Bridge**: Off-chain integration to recommend movies based on on-chain preferences.
6. **Social Sharing**: Features to easily share watchlists or reviews with other wallet addresses.

### Long-Term Vision
7. **Web3 Streaming Integration**: Partner with decentralized streaming platforms to automatically update the watchlist upon viewing.
8. **Decentralized UI Hosting**: Host the application frontend on IPFS.
9. **NFT Ticket Integration**: Link movie tickets (NFTs) to the watchlist to verify a user has seen a movie in theaters.
10. **DAO Governance**: Community-driven protocol improvements for new tracking features.

## Technical Requirements

- Soroban CLI / Stellar CLI
- Rust programming language (`wasm32-unknown-unknown` target)
- Stellar blockchain network (Testnet/Futurenet)

## Getting Started

Deploy the smart contract to Stellar's testnet network and interact with it using the three main functions:

- `add_film(title, genre)` - Add a new movie to your watchlist.
- `get_films()` - Retrieve all saved films from the contract.
- `remove_film(id)` - Remove a specific movie by its unique ID.

---

**Stellar Film Tracker DApp** - Securing Your Watchlist on the Blockchain