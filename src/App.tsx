import { Component, createContext } from "preact";
//import { useState } from "preact/hooks";
//import preactLogo from "./assets/preact.svg";
//import { invoke } from "@tauri-apps/api/core";
import Gun, { IGunInstance } from 'gun';
import "./App.css";


//export const gun = createContext(Gun(location.origin + '/gun'));
export const GunCtx = createContext(Gun());

class App extends Component {
        gun : IGunInstance

        constructor() {
                super();
                this.gun = Gun(location.origin + '/gun');
                //window.gun = this.gun;
        }

        render() {
                return (
                        <GunCtx.Provider value={this.gun}>
                                <main class="container">
                                </main>
                        </GunCtx.Provider>
                );
        }
}

export default App;
