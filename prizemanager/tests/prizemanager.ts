import * as anchor from "@coral-xyz/anchor";
import {BN} from "@coral-xyz/anchor"
import {Program} from "@coral-xyz/anchor";
import {Prizemanager, IDL} from "../target/types/prizemanager"
//import { ConstantProduct, LiquidityPair } from "constant-product-curve-wasm";
import {PublicKey, Commitment, Keypair, SystemProgram} from "@solana/web3.js"
import {
    ASSOCIATED_TOKEN_PROGRAM_ID as associatedTokenProgram,
    TOKEN_PROGRAM_ID as tokenProgram,
    createMint,
    createAccount,
    mintTo,
    getAssociatedTokenAddress,
    TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync
} from "@solana/spl-token"
import {randomBytes} from "crypto"
import {assert} from "chai"
import * as bs58 from "bs58";
import {ASSOCIATED_PROGRAM_ID} from "@coral-xyz/anchor/dist/cjs/utils/token";
import {wallet, wallet_two, wallet_three} from "../wallet/wallet"
import {min} from "bn.js";

const commitment: Commitment = "confirmed"; // processed, confirmed, finalized

describe("prize-manager", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const programId = new PublicKey("5KoYYoYuHL19B1EFjPNu9i3xcvbdxrqq5qyXHuasJfQ7");
    const program = new anchor.Program<Prizemanager>(IDL, programId, anchor.getProvider());

    // Set up our keys
    const initializer = Keypair.fromSecretKey(bs58.decode(wallet));
    const claimer_user = Keypair.fromSecretKey(bs58.decode(wallet_three))

    // Random seed
    const seed = new BN(randomBytes(8));

    // PDAs
    const prize_auth = PublicKey.findProgramAddressSync([Buffer.from("prize_auth")], program.programId)[0];
    const prize_config = PublicKey.findProgramAddressSync([Buffer.from("prize"), seed.toBuffer().reverse()], program.programId)[0];

    // Mints
    let prize_one_mint: PublicKey;

    // ATAs
    let particular_prize_vault: PublicKey;
    let admin_prize_vault: PublicKey;
    let claimer_ata: PublicKey;

    it("Create mints, tokens and ATAs", async () => {
        prize_one_mint = new PublicKey("BjwKL4x9TjoBgzkgBW14bzn1ocu7HX8up63qXG9AFWE9")
        particular_prize_vault = await getAssociatedTokenAddress(prize_one_mint, prize_auth, true, tokenProgram);
        admin_prize_vault = await getAssociatedTokenAddress(prize_one_mint, initializer.publicKey, false, tokenProgram)
        claimer_ata = await getAssociatedTokenAddress(prize_one_mint, claimer_user.publicKey, false, tokenProgram)
        console.log(particular_prize_vault.toBase58())
        console.log(admin_prize_vault.toBase58())
        console.log(claimer_ata.toBase58())
    })

    it("Initialize", async () => {
        try {
            const tx = await program.methods.initialize(
                seed,
                initializer.publicKey
            ).accounts({
                user: initializer.publicKey,
                prizeAuth: prize_auth,
                prizeMint: prize_one_mint,
                particularPrizeVault: particular_prize_vault,
                prizeConfig: prize_config,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
                systemProgram: SystemProgram.programId
            })
                .signers([initializer])
                .rpc()
            await confirmTx(tx)
            console.log("Your transaction signature", tx);
        } catch (e) {
            console.error(e);
        }
    });

    xit("PutPrizeOnVault", async () => {
        try {
            const tx = await program.methods.putPrizeOnVault(
            )
                .accounts({
                    user: initializer.publicKey,
                    prizeAuth: prize_auth,
                    prizeMint: prize_one_mint,
                    particularPrizeVault: particular_prize_vault,
                    adminPrizeVault: admin_prize_vault,
                    prizeConfig: prize_config,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
                    systemProgram: SystemProgram.programId
                })
                .signers([initializer])
                .rpc({skipPreflight: true})
            await confirmTx(tx)
            console.log("Your transaction signature", tx);
        } catch (e) {
            console.log(e)
        }
    })

    xit("PutPrizeBackOnAdminVault", async () => {
        try {
            const tx = await program.methods.givePrizeBackToVault()
                .accounts({
                    user: initializer.publicKey,
                    prizeAuth: prize_auth,
                    prizeMint: prize_one_mint,
                    particularPrizeVault: particular_prize_vault,
                    adminPrizeVault: admin_prize_vault,
                    prizeConfig: prize_config,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
                    systemProgram: SystemProgram.programId
                })
                .signers([initializer])
                .rpc({skipPreflight: true})
            await confirmTx(tx)
            console.log("Your transaction signature", tx);
        } catch (e) {
            console.log(e)
        }
    })

    xit("claimPrize", async () => {
        try {
            const tx = await program.methods.claimPrize()
                .accounts({
                    user: claimer_user.publicKey,
                    prizeMint: prize_one_mint,
                    particularPrizeVault: particular_prize_vault,
                    claimerAta: claimer_ata,
                    prizeAuth: prize_auth,
                    prizeConfig: prize_config,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .signers([claimer_user , initializer])
                .rpc({skipPreflight: true})
            await confirmTx(tx)
            console.log("Your transaction signature", tx);
        } catch (e) {
            console.log(e)
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