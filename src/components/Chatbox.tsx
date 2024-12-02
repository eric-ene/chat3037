import './Chatbox.css'
import {ChangeEvent, KeyboardEvent, useEffect, useRef, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Message, MessagePayload} from "../Classes.ts";
import {listen} from "@tauri-apps/api/event";

export default function Chatbox(props: { other: string, otherId: string }) {
    const inputRef = useRef<HTMLInputElement | null>(null);
    const [messages, setMessages] = useState<Message[]>([])
    const [currentMessage, setCurrentMessage] = useState("");
    const [id, setId] = useState(0);
    let scrollRef = useRef<HTMLDivElement | null>(null)

    useEffect(() => {
        const listener = listen<MessagePayload>('incoming-message', (evt) => {
            let msg = new Message()
            msg.sender = props.other;
            msg.content = evt.payload.content;
            
            setId(id => id + 1);
            msg.id = id;
            
            appendMessage(msg);
        })
        
        return () => {
            listener.then(f => f())
        }
    }, []);

    useEffect(() => {
        if (scrollRef.current) {
            // @ts-ignore
            scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
        }
    }, [messages]);
    
    function generateMessage(dst: string, content: string) {
        let retval = new Message();
        
        retval.dst = dst;
        retval.sender = 'You';
        retval.content = content;

        setId(id => id + 1);
        retval.id = id;
        
        return retval;
    }
    
    function appendMessage(message: Message) {
        setMessages(oldArray => [...oldArray, message]);
    }
    
    function handleChange(evt: ChangeEvent<HTMLInputElement>) {
        setCurrentMessage(evt.target.value);
    }
    
    function handleKeyDown(evt: KeyboardEvent<HTMLInputElement>) {
        if (evt.code === 'Enter' && !isWhitespace(currentMessage)) {
            let msg = generateMessage(props.otherId, currentMessage);
            
            setCurrentMessage('');

            appendMessage(msg);
            
            invoke('send_message', { message: msg })
                .then(() => {})
                .catch((err) => console.log(err))
            
            return;
        }
    }
    
    return (
        <div className={"chatbox"} onClick={() => inputRef.current?.focus() }>
            <div className={"chat-window"} ref={scrollRef}>
                {
                    messages.map((message) => {
                       return <div className={"message sidebyside"}>
                           <p key={message.id} className={"msg-name"}>{message.sender}:</p>
                           <p className={"msg-text"}>{message.content}</p>
                       </div>    
                    })
                }
            </div>
            <p className={"send"}>&gt;</p>
            <input ref={inputRef} className={"send"} onChange={handleChange} onKeyDown={handleKeyDown} value={currentMessage}/>
        </div>
    );
}

function isWhitespace(str: string) {
    return str.replace(/\s/g, '').length == 0;
}