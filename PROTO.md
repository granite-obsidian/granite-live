# Protocol

This will be sent by the client to the server and from the server to the client every update (~500ms)

```ts
{
    // Only sent by Client
    "client": {
        "token": string?,
        "fetchAll": boolean?,
    }?,
    // Only sent by Server
    "server": {},
    "files": [
        "filepath": string,
        "filehash": string,
        "diff": [
            "line": number,
            "text": string,
        ]?,
        "fullfile": string?,
    ]?
}
```

```rs
type Packet = Option<Vec<File>>;

struct Packet = {
    clientData: Optional<ClientData>,
    serverData: Optional<ServerData>,
    files: Optional<Vec<File>>,
};

struct File = {
    filepath: String,
    filehash: String,
    diff: Option<Vec<LineDiff>>,
    fullfile: Option<String>,
}

struct LineDiff = {
    line: u32,
    text: String,
}
```
