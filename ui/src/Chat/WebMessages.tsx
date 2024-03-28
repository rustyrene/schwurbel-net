interface Props {
    messages: WebMessage[];
}

function WebMessages({ messages }: Props) {
    const getMessage = (message: WebMessage) => {
        return <p className={`px-3 py-1 fs-3 rounded-4 text-${message.isAuthor ? "end" : "start"} text-bg-${message.isAuthor ? "primary" : "danger"}`}>{message.msg}</p>
    }

  return (
    <>
        <div className="d-flex flex-column">
            {messages.map(message => <div className={`w-25 align-self-${message.isAuthor ? "end" : "start"}`}>{getMessage(message)}</div>)}
        </div>
    </>
  )
}

export default WebMessages