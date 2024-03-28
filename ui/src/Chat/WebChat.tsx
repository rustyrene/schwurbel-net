interface Props {
    onSendMessage: () => void;
}

function WebChat({ onSendMessage }: Props) {
  return (
    <button onClick={onSendMessage}>Send Message</button>
  )
}

export default WebChat