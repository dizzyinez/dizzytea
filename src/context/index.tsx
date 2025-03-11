import { FunctionComponent } from "preact";
import Router from "preact-router";
import { SurrealProvider } from "./components/SurrealProvider";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import Surreal from "surrealdb";
import { surrealdbWasmEngines } from "@surrealdb/wasm";


const customClient = new Surreal({
        engines: surrealdbWasmEngines(),
});
await customClient.connect("indxdb://");

export const AppProviders: FunctionComponent = ({ children }) => {
        return (
                <QueryClientProvider client={new QueryClient()}>
                        <SurrealProvider client={customClient} endpoint="indxdb://">
                                {children}
                        </SurrealProvider>
                </QueryClientProvider>
        )
}
