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

let abstractPath = "../hidden"

let useClient protocolType = 
    printfn $"{protocolType}"
    
    try
        let clientSocket = new Socket(AddressFamily.Unix, SocketType.Stream, protocolType)
        let endpoint = new UnixDomainSocketEndPoint(abstractPath);
        clientSocket.Connect(endpoint);
        printfn "Connected to server!"
    
        let message = "Hello from client!"
        let data = Encoding.UTF8.GetBytes(message)
        clientSocket.Send data |> printfn "Send bytes: {%i}"
    
        clientSocket.Close()
    with
    | e -> printfn "Protocol does not works {%A}. Exception message: {%s}" protocolType e.Message

    ()


useClient ProtocolType.Tcp
useClient ProtocolType.Udp
useClient ProtocolType.IP
useClient ProtocolType.IPv4
useClient ProtocolType.IPv6
useClient ProtocolType.Raw
useClient ProtocolType.Unknown
useClient ProtocolType.Unspecified