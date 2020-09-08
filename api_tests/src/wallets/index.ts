import { sleep } from "../utils";
import { BitcoinWallet } from "./bitcoin";
import { EthereumWallet } from "./ethereum";
import { Asset } from "../asset";
import { LightningWallet } from "./lightning";
import { Logger } from "log4js";
import pTimeout from "p-timeout";

export interface AllWallets {
    bitcoin?: BitcoinWallet;
    ethereum?: EthereumWallet;
    lightning?: LightningWallet;
}

export interface Wallet {
    MaximumFee: bigint;
    getBalanceByAsset(asset: Asset): Promise<bigint>;
}

export class Wallets {
    constructor(private readonly wallets: AllWallets) {}

    get bitcoin(): BitcoinWallet {
        return this.getWalletForLedger("bitcoin");
    }

    get ethereum(): EthereumWallet {
        return this.getWalletForLedger("ethereum");
    }

    get lightning(): LightningWallet {
        return this.getWalletForLedger("lightning");
    }

    public getWalletForLedger<K extends keyof AllWallets>(
        name: K
    ): AllWallets[K] {
        const wallet = this.wallets[name];

        if (!wallet) {
            throw new Error(`Wallet for ${name} is not initialised`);
        }

        return wallet;
    }
}

export async function pollUntilMinted(
    getBalance: () => Promise<bigint>,
    minimumBalance: bigint
): Promise<void> {
    const timeout = 10; // minting shouldn't take longer than 10 seconds
    const error = new Error(`Minting failed after ${timeout} seconds`);
    Error.captureStackTrace(error);

    let currentBalance = await getBalance();

    const poller = async () => {
        while (currentBalance < minimumBalance) {
            await sleep(500);
            currentBalance = await getBalance();
        }
    };

    await pTimeout(poller(), timeout * 1000, error);
}

export function newBitcoinStubWallet(logger: Logger): BitcoinWallet {
    return newStubWallet(
        {
            MaximumFee: BigInt(0),
            getAddress: () =>
                Promise.resolve("bcrt1qq7pflkfujg6dq25n73n66yjkvppq6h9caklrhz"),
            getBalance: () => Promise.resolve(BigInt(0)),
            mintToAddress: (
                _minimumExpectedBalance: bigint,
                _toAddress: string
            ) => Promise.resolve(),
        },
        logger
    );
}

export function newEthereumStubWallet(logger: Logger): EthereumWallet {
    return newStubWallet(
        {
            getAccount: () => "0x00a329c0648769a73afac7f9381e08fb43dbea72",
            getErc20Balance: (
                _contractAddress: string,
                _decimals?: number
            ): Promise<bigint> => Promise.resolve(BigInt(0)),
            mintErc20: (_quantity: bigint, _tokenContract: string) =>
                Promise.resolve(),
        },
        logger
    );
}

export function newLightningStubWallet(logger: Logger): LightningWallet {
    return newStubWallet(
        {
            getPubkey: () =>
                Promise.resolve(
                    "02ed138aaed50d2d597f6fe8d30759fd3949fe73fdf961322713f1c19e10036a06"
                ),
            getBalance: () => Promise.resolve(BigInt(0)),
        },
        logger
    );
}

function newStubWallet<W extends Wallet, T extends Partial<W>>(
    stubs: T,
    logger: Logger
): W {
    const stubWallet: Partial<W> = {
        ...stubs,
        mint: (_: Asset) => {
            logger.warn("StubWallet doesn't mint anything");
        },
        getBalanceByAsset: async (asset: Asset) => {
            logger.warn(
                "StubWallet always returns 0 balance for asset",
                asset.name
            );

            return Promise.resolve(0);
        },
    };

    return (stubWallet as unknown) as W;
}
