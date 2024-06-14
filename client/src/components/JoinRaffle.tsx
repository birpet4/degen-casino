import React, { useEffect, useState } from 'react';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { ConfirmOptions, PublicKey, SystemProgram } from '@solana/web3.js';
import { AnchorProvider, Program, utils } from '@project-serum/anchor';
import idl from '../idl.json'; // Import your IDL file

const programID = new PublicKey(idl.address);
const network = 'https://api.devnet.solana.com';
const opts: ConfirmOptions  = { preflightCommitment: "processed" };

const JoinRaffle: React.FC = () => {
    const { connection } = useConnection();
    const { publicKey, sendTransaction } = useWallet();
    const [program, setProgram] = useState<Program | null>(null);

    useEffect(() => {
        if (publicKey) {
            const provider = new AnchorProvider(connection, window.solana, opts);
            const program = new Program(idl as any, programID, provider);
            setProgram(program);
        }
    }, [publicKey, connection]);

    const joinRaffle = async () => {
        if (!program || !publicKey) return;

        try {
            const [rafflePDA] = await PublicKey.findProgramAddress(
                [utils.bytes.utf8.encode("raffle"), publicKey.toBuffer()],
                programID
            );

            const tx = await program.rpc.joinRaffle({
                accounts: {
                    raffle: rafflePDA,
                    user: publicKey,
                    systemProgram: SystemProgram.programId,
                },
            });

            console.log("Transaction signature", tx);
        } catch (err) {
            console.error("Transaction error: ", err);
        }
    };

    return (
        <div>
            <button onClick={joinRaffle} disabled={!publicKey}>Join Raffle</button>
        </div>
    );
};

export default JoinRaffle;
