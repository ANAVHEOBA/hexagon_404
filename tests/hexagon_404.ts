import * as anchor from "@coral-xyz/anchor";
import { Program, Idl } from "@coral-xyz/anchor";
import { Hexagon404 } from "../target/types/hexagon_404";

// Define the ConfigAccount interface to match the account structure
interface ConfigAccount {
  authority: anchor.web3.PublicKey;
  tokenName: string;          // Changed from token_name
  tokenSymbol: string;        // Changed from token_symbol
  totalSupply: anchor.BN;     // Changed from total_supply
  decimals: number;
  burnRate: number;           // Changed from burn_rate
  lastMintTimestamp: anchor.BN;  // Changed from last_mint_timestamp
  initialized: boolean;
  bump: number;
}

// Define the TokenAccountData interface to match the account structure
interface TokenAccountData {
  owner: anchor.web3.PublicKey;
  balance: anchor.BN;
  bump: number;
}

describe("hexagon_404", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Create program instance using the workspace and explicit type
  const program = anchor.workspace.Hexagon404 as Program<Hexagon404>;

  it("Checks token supply and holder", async () => {
    try {
      // Initialize the authority (test wallet)
      const authority = provider.wallet.publicKey;

      // Get the config PDA
      const [configPda, configBump] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("config")],
        program.programId
      );
      console.log("Config PDA:", configPda.toString());

      // Get the token account PDA first
      const [tokenAccountPda, tokenAccountBump] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("token_account"), authority.toBuffer()],
        program.programId
      );
      console.log("\nToken Account PDA:", tokenAccountPda.toString());

      // Initialize the config account
      await program.methods
        .initialize("TestToken", "TST")
        .accounts({
          authority,
          config: configPda,
          tokenAccount: tokenAccountPda,
          systemProgram: anchor.web3.SystemProgram.programId
        })
        .rpc();

      // Fetch and log the config account
      const configAccount = await program.account.config.fetch(configPda);
      console.log("\nConfig Account Details:");
      console.log("Authority:", configAccount.authority.toString());
      console.log("Token Name:", configAccount.tokenName);
      console.log("Token Symbol:", configAccount.tokenSymbol);
      console.log("Total Supply:", configAccount.totalSupply.toString());
      console.log("Decimals:", configAccount.decimals);
      console.log("Burn Rate:", configAccount.burnRate);
      console.log("Last Mint Timestamp:", configAccount.lastMintTimestamp.toString());
      console.log("Initialized:", configAccount.initialized);
      console.log("Bump:", configAccount.bump);

      // Fetch and log the token account
      const tokenAccount = await program.account.tokenAccount.fetch(tokenAccountPda);
      console.log("\nToken Account Details:");
      console.log("Owner:", tokenAccount.owner.toString());
      console.log("Balance:", tokenAccount.balance.toString());
      console.log("Bump:", tokenAccount.bump);

    } catch (error) {
      console.error("\nDetailed Error:");
      console.error("Error type:", error.constructor.name);
      console.error("Error message:", error.message);
      console.error("Full error:", error);
      throw error;
    }
  });
});