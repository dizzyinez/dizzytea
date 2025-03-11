import { FunctionComponent } from "preact";
import Router from "preact-router";
import { Client, fetchExchange, Provider } from "@urql/preact";
import { offlineExchange } from "@urql/exchange-graphcache";
import { makeDefaultStorage } from "@urql/exchange-graphcache/default-storage";
import schema from '../introspection.json'

const storage = makeDefaultStorage({
        idbName: 'graphcache-v3',
        maxAge: 60,
})

const cache = offlineExchange({
        schema,
        storage,
})

const client = new Client({
        url: 'http://localhost:3000/graphql',
        exchanges: [cache, fetchExchange]
});

export const AppProviders: FunctionComponent = ({ children }) => {
        return (
                <Provider value={client}>
                        <Router>
                                {children}
                        </Router>
                </Provider>
        )
}
