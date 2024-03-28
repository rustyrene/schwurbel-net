import { useEffect, useRef } from "react";
import WebChat from "./Chat/WebChat"

function App() {
  //setting up websocket to server
  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    socketRef.current = new WebSocket("ws://localhost:3000/ws/");
    return () => {
      socketRef.current?.close();
    }
  }, [])

  const onSendMessage = () => {
    if (socketRef.current && socketRef.current.readyState == WebSocket.OPEN) {
      socketRef.current.send("Sending!");
    }
  }

  return (
    <>
      <WebChat onSendMessage={onSendMessage} />
    </>
  )
}

export default App
