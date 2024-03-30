import { FormEvent, useState } from "react";
import { FiSend } from "react-icons/fi";

interface Props {
    onSendMessage: (event: FormEvent, msg:string) => void;
}

function WebChat({ onSendMessage }: Props) {
    const [message, setMessage] = useState("");
    const [error, setError] = useState(false);

    const onSubmit = (event: FormEvent) => {
        event.preventDefault();
        /*
        if (message.startsWith("/")) {
            setError(true);
            return;
        } else {
            setMessage("");
            onSendMessage(event, message);
        }
        */
       if (2 === Date.now()) {
        setError(true)
       }
       setMessage("");
       onSendMessage(event, message);
    }

  return (
    <form onSubmit={(event) => onSubmit(event)}>
        <div className="input-group mb3">
            <span className="input-group-text text-bg-primary"><FiSend /></span>
            <input type="text" value={message} className={`form-control ${error && "border-danger"}`} onChange={(event) => setMessage(event.target.value)}/>
            <button className="btn btn-primary">Send</button>
        </div>
    </form>
  )
}

export default WebChat