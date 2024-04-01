import { FormEvent, useState } from "react";
import { FiSend } from "react-icons/fi";

interface Props {
    onSendMessage: (event: FormEvent, msg:string) => void;
}

function WebChat({ onSendMessage }: Props) {
    const [message, setMessage] = useState("");
    const [error, setError] = useState(false);

    const onSubmit = (event: FormEvent) => {
        if(message === "") {
            setError(true);
            return;
        }
        event.preventDefault();
       setMessage("");
       onSendMessage(event, message);
    }

    const handleChange = (input: string) => {
        setError(false);
        setMessage(input);
    }

  return (
    <form onSubmit={(event) => onSubmit(event)}>
        <div className="input-group mb3">
            <span className="input-group-text text-bg-primary"><FiSend /></span>
            <input type="text" value={message} className={`form-control ${error && "border-danger"}`} onChange={(event) => handleChange(event.target.value)}/>
            <button className="btn btn-primary">Send</button>
        </div>
    </form>
  )
}

export default WebChat