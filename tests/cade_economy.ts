import * as anchor from "@coral-xyz/anchor";
import {BN} from "@coral-xyz/anchor"
import {Program} from "@coral-xyz/anchor";
import {Newamm, IDL} from "../target/types/newamm"
//import { ConstantProduct, LiquidityPair } from "constant-product-curve-wasm";
import {PublicKey, Commitment, Keypair, SystemProgram} from "@solana/web3.js"
import {
  ASSOCIATED_TOKEN_PROGRAM_ID as associatedTokenProgram,
  TOKEN_PROGRAM_ID as tokenProgram,
  createMint,
  createAccount,
  mintTo,
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token"
import {randomBytes} from "crypto"
import {assert} from "chai"
import * as bs58 from "bs58";
import {ASSOCIATED_PROGRAM_ID} from "@coral-xyz/anchor/dist/cjs/utils/token";
//import {walletKey} from "../Keys/keys"
import {min} from "bn.js";

const commitment: Commitment = "confirmed"; // processed, confirmed, finalized

describe("anchor-amm-2023", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const programId = new PublicKey("Ahjf4V5Nb9GQBoeCfeAMpk26A4ACk7NkBaLWfbFX5Bxs");
  const program = new anchor.Program<Newamm>(IDL, programId, anchor.getProvider());

  // Set up our keys
  const [initializer] = [new Keypair()];
  //const initializer = Keypair.fromSecretKey(bs58.decode(walletKey));

  // Random seed
  const seed = new BN(randomBytes(8));

  // PDAs
  const auth = PublicKey.findProgramAddressSync([Buffer.from("auth")], program.programId)[0];
  const config = PublicKey.findProgramAddressSync([Buffer.from("config"), seed.toBuffer().reverse()], program.programId)[0];

  // Mints
  let mint_x: PublicKey;
  let mint_lp = PublicKey.findProgramAddressSync([Buffer.from("lp"), config.toBuffer()], program.programId)[0];

  // ATAs
  let initializer_x_ata: PublicKey;
  let initializer_lp_ata: PublicKey;
  let vault_x_ata: PublicKey;
  let vault_lp_ata: PublicKey;

  it("Airdrop", async () => {
    await Promise.all([initializer].map(async (k) => {
      return await anchor.getProvider().connection.requestAirdrop(k.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL)
    })).then(confirmTxs);
  });

  it("Create mints, tokens and ATAs", async () => {
    // Create mints and ATAs
    let [u1] = await Promise.all([initializer].map(async (a) => {
      return await newMintToAta(anchor.getProvider().connection, a)
    }))
    mint_x = u1.mint;
    initializer_x_ata = u1.ata
    console.log("mint_x",u1.mint.toBase58())
    console.log("init_x_ata",u1.ata.toBase58())
    initializer_lp_ata = await getAssociatedTokenAddress(mint_lp, initializer.publicKey, false, tokenProgram);
    // Create take ATAs
    vault_x_ata = await getAssociatedTokenAddress(mint_x, auth, true, tokenProgram);
    vault_lp_ata = await getAssociatedTokenAddress(mint_lp, auth, true, tokenProgram);
  })

  it("Initialize", async () => {
    try {
      const tx = await program.methods.initialize(
          seed,
          initializer.publicKey
      )
          .accounts({
            auth,
            user: initializer.publicKey,
            mintX: mint_x,
            mintLp: mint_lp,
            vaultX: vault_x_ata,
            vaultLp : vault_lp_ata,
            config,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          })
          .signers([
            initializer
          ]).rpc();
      await confirmTx(tx);
      console.log("Your transaction signature", tx);
    } catch (e) {
      console.error(e);
    }
  });

    it("Initialize_LP", async () => {
        try {
            const tx = await program.methods.lpInitialize(
                seed,
                initializer.publicKey
            )
                .accounts({
                    auth,
                    user: initializer.publicKey,
                    mintLp: mint_lp,
                    vaultLp: vault_lp_ata,
                    config,
                    lpConfig : lp_config,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
                    systemProgram: SystemProgram.programId
                })
                .signers([
                    initializer
                ]).rpc({skipPreflight: true});
            await confirmTx(tx);
            console.log("Your transaction signature", tx);
        } catch (e) {
            console.error(e);
        }
    });

    it("MintLP", async () => {
        try {
            const tx = await program.methods.mintLp(
                new BN(5_000_000_000)
            )
                .accountsStrict({
                    auth,
                    user : initializer.publicKey,
                    mintX: mint_x,
                    mintLp: mint_lp,
                    vaultLp: vault_lp_ata,
                    lpConfig : lp_config,
                    config,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
                    systemProgram: SystemProgram.programId
                })
                .signers([
                    initializer
                ]).rpc();
            await confirmTx(tx);
            console.log("Your transaction signature", tx);
        } catch (e) {
            console.error(e);
        }
    })


  it("Swap X for LP", async () => {
    try {
      const tx = await program.methods.swap(
          new BN(5_000_000),
          new BN(Math.floor(new Date().getTime() / 1000) + 600)
      )
          .accountsStrict({
            auth,
            user: initializer.publicKey,
            mintX: mint_x,
            mintLp: mint_lp,
            userVaultX: initializer_x_ata,
            userVaultLp: initializer_lp_ata,
            vaultX: vault_x_ata,
            config,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          })
          .signers([
            initializer
          ]).rpc(
              {skipPreflight:true}
          );
      await confirmTx(tx);
      console.log("Your transaction signature", tx);
    } catch (e) {
      let err = e as anchor.AnchorError;
      console.error(e);
      if (err.error.errorCode.code !== "InvalidAuthority") {
        throw (e)
      }
    }
  });

  it("Pay to play" , async () => {
    try {
      const tx = await program.methods.pay(
          new BN(5_000_000)
      )
          .accountsStrict({
            auth,
            user : initializer.publicKey,
            mintLp : mint_lp,
            vaultLp : vault_lp_ata,
            userVaultLp : initializer_lp_ata,
            config,
            tokenProgram : TOKEN_PROGRAM_ID,
            associatedTokenProgram : ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram : SystemProgram.programId
          })
          .signers([
            initializer
          ]).rpc(

          );
      await confirmTx(tx);
      console.log("Your transaction signature", tx);
    } catch (e) {
      let err = e as anchor.AnchorError;
      console.error(e);
      if (err.error.errorCode.code !== "InvalidAuthority") {
        throw (e)
      }
    }
  })

// Helpers
  const confirmTx = async (signature: string) => {
    const latestBlockhash = await anchor.getProvider().connection.getLatestBlockhash();
    await anchor.getProvider().connection.confirmTransaction(
        {
          signature,
          ...latestBlockhash,
        },
        commitment
    )
  }

  const confirmTxs = async (signatures: string[]) => {
    await Promise.all(signatures.map(confirmTx))
  }

  const newMintToAta = async (connection, minter: Keypair): Promise<{ mint: PublicKey, ata: PublicKey }> => {
    const mint = await createMint(connection, minter, minter.publicKey, null, 6)
    // await getAccount(connection, mint, commitment)
    const ata = await createAccount(connection, minter, mint, minter.publicKey)
    const signature = await mintTo(connection, minter, mint, ata, minter, 21e8)
    await confirmTx(signature)
    return {
      mint,
      ata
    }
  }
})