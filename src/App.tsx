import "./App.css";
import Header from "./components/Header.tsx";
import Body from "./components/Body.tsx";
import {useRef} from "react";
import {getCurrentWindow} from "@tauri-apps/api/window";

function App() {
    const window = getCurrentWindow()
    window.setResizable(false).then();
    
    const nameRef = useRef<HTMLInputElement | null>(null);
    
    return (
        <div id={"app"}>
            <Header nameRef={nameRef}/>
            <Body nameRef={nameRef}/>
        </div>
    );
}

export default App;
