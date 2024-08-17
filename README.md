# Crypton Payment Processor

Crypton is an open-source crypto payment processor designed to leverage the advantages of Layer 2 solutions, providing a truly disruptive alternative to traditional fiat payment processors. Crypton aims to offer ultra-low transaction fees and a seamless experience for users and ecommerce businesses alike, without compromising on the benefits of decentralization.

## Project Overview

### Introduction

With the rise of Layer 2 scaling solutions, cryptocurrency is now able to rival credit cards in terms of transaction processing time and fees. Existing crypto payment processors, however, are too similar to fiat solutions and fail to offer the level of disruption that crypto promises. Notably, these processors often have fees on the order of 1% and require Know Your Customer (KYC) compliance similar to traditional systems.

Crypton addresses these issues by providing a crypto-native solution that significantly reduces transaction fees—by as much as 100 times—and removes the need for cumbersome KYC processes. By requiring only a wallet connection for users, Crypton offers a simple, fast, and secure payment experience.

### Key Features

- **Open Source**: Crypton is fully open source, allowing for community-driven development and auditability. This ensures that the processor can evolve to support new cryptocurrencies as they emerge.
- **Low Fees**: With transaction fees drastically lower than traditional processors, Crypton provides a cost-effective solution for both merchants and customers.
- **No KYC**: Users only need to connect their wallet, making the signup and payment process frictionless.
- **Multi-Chain Support**: Crypton supports multiple Layer 2 solutions, including Polygon, Arbitrum, Zksync, and Optimism, providing flexibility for users to choose their preferred payment method.
- **Seamless Integration**: A simple JavaScript snippet allows ecommerce businesses to integrate Crypton into their websites with minimal effort. The payment processing is handled via a frontend and backend app, ensuring a smooth transaction flow.

## How It Works

### Components

1. **Ecommerce Example Site**:
   - **Frontend**: Built with Svelte, it uses Web3Modal to prompt customers to connect their wallets. Web3.js is used for blockchain interactions, including chain switching.
   - **Price Data**: Fetches token price data using Covalent.
   - **Payment Options**: Supports payments using multiple Ethereum Layer 2 solutions.

2. **Crypton App**:
   - **Frontend**: Utilizes the same stack as the ecommerce example site, ensuring a consistent user experience.
   - **Backend**: Although initially planned, the backend is not fully implemented. Instead, transaction data is fetched and displayed using block explorers.

### Integration

- **JavaScript Snippet**: Ecommerce clients can add a simple JS snippet to their site, enabling crypto payments for their customers. 
- **Webhooks**: Once a transaction is confirmed, a webhook is called to notify the ecommerce client's backend, ensuring they can process the order.

### Technology Stack

- **Frontend**: Svelte, Web3Modal, Web3.js
- **Backend**: Block explorer APIs (as a temporary solution)
- **Data Fetching**: Covalent API for price data
- **Supported Chains**: Ethereum Layer 2 solutions (Polygon, Arbitrum, Zksync, Optimism)

## Future Development

The current iteration of Crypton has laid the foundation for a powerful crypto payment processor, but there are still several areas for future development:

- **Complete Backend Implementation**: Finalize the backend to handle transaction data directly, without relying on block explorers.
- **Enhanced Security**: Implement additional security measures to ensure the robustness of the payment system.
- **Broader Token Support**: Extend support to more cryptocurrencies and Layer 2 solutions.
- **Merchant Tools**: Develop additional tools and features for ecommerce clients, such as analytics and customer management.

## Contributing

Crypton is open to contributions from developers, designers, and enthusiasts. Whether you're interested in improving the existing codebase, adding new features, or simply providing feedback, your contributions are welcome.

## License

Crypton is released under the MIT License, allowing for flexibility in its use and distribution.
