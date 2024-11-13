
import "./App.css";
import Header from "./components/Header.tsx";
import Body from "./components/Body.tsx";
import {useState} from "react";
import {getCurrentWindow} from "@tauri-apps/api/window";

function App() {
    const window = getCurrentWindow()
    window.setResizable(false).then();
    
    return (
        <div id={"app"}>
            <Header/>
            <Body/>
        </div>
    );
}

export default App;
