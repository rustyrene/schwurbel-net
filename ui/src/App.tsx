import { FormEvent, useEffect, useRef } from "react";
import WebChat from "./Chat/WebChat"

function App() {
  //setting up websocket to server
  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    socketRef.current = new WebSocket("ws://localhost:3000/ws/");
    socketRef.current.addEventListener("message", function(event) {
      console.log(event.data);
    })
    return () => {
      socketRef.current?.close();
    }
  }, [])

  const onSendMessage = (event: FormEvent, msg:string) => {
    event.preventDefault();
    if (socketRef.current && socketRef.current.readyState == WebSocket.OPEN) {
      socketRef.current.send(msg);
    }
  }

  return (
    <>
      <WebChat onSendMessage={onSendMessage} />
    </>
  )
}

export default App
