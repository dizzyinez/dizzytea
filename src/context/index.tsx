import { FunctionComponent } from "preact";
import Router from "preact-router";
import { GunProvider } from "./components/GunProvider";
import { cacheExchange, Client, fetchExchange, Provider } from "@urql/preact";

const client = new Client({
        url: 'http://localhost:3000/graphql',
        exchanges: [cacheExchange, fetchExchange]
});

export const AppProviders: FunctionComponent = ({ children }) => {
        //<GunProvider peerUrls={[location.origin + '/gun']}>
        return (
                <div>
                        <Provider value={client}>
                                {children}
                        </Provider>
                </div>
        )
        //return (
        //        <Router>
        //                <GunProvider peerUrls={[]}>
        //                        {children}
        //                </GunProvider>
        //        </Router>
        //)
}
