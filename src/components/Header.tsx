import {invoke} from "@tauri-apps/api/core";
import {useEffect, useState} from "react";


export default function Header() {
    const [id, setId] = useState("loading...");
    
    useEffect(() => {
        async function applyId() {
            const val = await invoke<string>("generate_identifier")
            setId(val)
        }
        
        applyId()
    });
    
    return (
        <div className={"topbar"}>
            <h1 id={"title"}>Chat3037</h1>
            <div id={"login"}>
                <p>Name:</p>
                <input type={"text"} defaultValue={"user"} className={"no-border"}/>
            </div>
            <p>Your ID: {id}</p>
        </div>
    );
}
