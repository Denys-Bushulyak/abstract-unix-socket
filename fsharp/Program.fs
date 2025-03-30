open System.Net.Sockets
open System.Text

type Message = {
    Name: string
    Message: string
}

let handle message = 
    { 
        Name = "Denys"
        Message = message 
    } 

let useClient path socketType protocolType = 
    
    let clientSocket = new Socket(AddressFamily.Unix, socketType, protocolType)
    let endpoint = new UnixDomainSocketEndPoint(path);
    clientSocket.Connect(endpoint);
    printfn "Connected to server!"
    printfn $"{protocolType}"

    let message = "Hello from client!"
    let data = Encoding.UTF8.GetBytes(message)
    clientSocket.Send data |> printfn "Send bytes: {%i}"

    clientSocket.Close()
    
    ()

useClient "\x00hidden" SocketType.Stream  ProtocolType.Unspecified

