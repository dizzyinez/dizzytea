import { Component } from "preact";
//import { useState } from "preact/hooks";
//import preactLogo from "./assets/preact.svg";
//import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { AppProviders } from "./context";
import { Music } from "./Music";

class App extends Component {
        render() {
                return (
                        <AppProviders>
                                <main class="container">
                                        <Music />
                                </main>
                        </AppProviders>
                );
        }
}

export default App;
