import { IGunInstance } from "gun";

export interface IGunContext {
    connect: (additionalPeers?: string[]) => IGunInstance;
    disconnect: () => void;
    gun: IGunInstance;
}
