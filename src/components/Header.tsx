import {invoke} from "@tauri-apps/api/core";
import {useEffect, useState} from "react";


export default function Header() {
    const [id, setId] = useState("loading...");
    
    useEffect(() => {
        async function applyId() {
            invoke<string>("get_identifier")
                .then((val) => {
                    setId(val);
                    console.log(val)
                })
                .catch((err) => console.error(err))
        }
        
        applyId()
            .catch(console.error);
    });
    
    return (
        <div className={"topbar"}>
            <h1 id={"title"}>Chat3037</h1>
            <div id={"login"}>
                <p>Name:</p>
                <input type={"text"} defaultValue={"user"} className={"no-border"}/>
            </div>
            <p>Your ID is "{id}"</p>
        </div>
    );
}
