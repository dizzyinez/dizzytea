import Gun from "gun/gun";
//import React, { FC, useCallback, useEffect, useState } from "react";
import { IGunProvider, IMesh, IPeer } from "./types";
import { FunctionComponent } from "preact";
import { useCallback, useEffect, useState } from "preact/hooks";
import { IGunChain, IGunInstance } from "gun";
import { GunContext } from "../../gun";

export const GunProvider: FunctionComponent<IGunProvider> = ({ children, peerUrls, connectOnMount = false }) => {
    const [gun, setGun] = useState<IGunInstance>(Gun());

    useEffect(() => {
        if (connectOnMount) {
            connect();
        }
    }, []);

    const connect = useCallback(
        (additionalPeers: string[] = []) => {
            if (!gun) {
                const instance = Gun([...peerUrls, ...additionalPeers]);
                setGun(instance);
            }

            return gun;
        },
        [peerUrls, gun, setGun]
    );

    const disconnect = useCallback(() => {
        if (gun) {
            //const mesh: IMesh = gun.get("opt.mesh");
            //const peers: Record<string, IPeer> = gun.back("opt.peers");

            //Object.keys(peers).forEach(id => {
            //    const peer = peers[id];

            //    if (peer) {
            //        peer.retry = -1;

            //        if (peer.wire?.close) {
            //            peer.wire.close();
            //        }

            //        mesh.bye(id);
            //    }
            //});

            //setGun(null);
        }
    }, [gun, setGun]);

    return <GunContext.Provider value={{ connect, disconnect, gun }}>{children}</GunContext.Provider>;
};
