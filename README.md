# Moraq Leagues Near Edition

Moraq Leagues Near Edition is a project that combines AI agents, blockchain technology, and a crypto price betting platform. It leverages the NEAR blockchain for smart contract functionality and uses AI agents to enhance the overall experience.

## Project Structure

The project is divided into three main components:

1. **Backend (Rust)**
   - Location: `/backend`
   - Features:
     - AI agent management system
     - Launch and control multiple AI agents
     - Interaction with the NEAR blockchain

2. **Smart Contracts (NEAR)**
   - Location: `/contracts`
   - Contains NEAR smart contracts for blockchain interactions

3. **Frontend (Next.js)**
   - Location: `/frontend`
   - Tech Stack:
     - Next.js
     - Tailwind CSS
     - shadcn UI components
   - Features:
     - User interface for interacting with the Moraq Leagues system
     - Integration with NEAR blockchain

## Prerequisites

- Rust
- Node.js
- NEAR CLI
- (Any other specific tools or SDKs required)

## Getting Started

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/moraq-leagues-near-edition.git
   cd moraq-leagues-near-edition
   ```

2. Set up the backend:
   ```
   cd backend
   cargo build
   ```

3. Deploy the NEAR contracts:
   ```
   cd ../contracts
   near deploy
   ```

4. Set up and run the frontend:
   ```
   cd ../frontend
   npm install
   npm run dev
   ```