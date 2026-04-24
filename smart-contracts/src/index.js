import { Server, Keypair, TransactionBuilder, Networks, Asset, Operation } from 'stellar-sdk';

// Bare-bones Stellar project setup placeholder.
// No payment logic is implemented here.

export const horizonServer = new Server('https://horizon-testnet.stellar.org');
export const stellarKeypair = Keypair;
export const stellarTransactionBuilder = TransactionBuilder;
export const stellarNetworks = Networks;
export const stellarAsset = Asset;
export const stellarOperation = Operation;
