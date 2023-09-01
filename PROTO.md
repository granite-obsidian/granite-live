# Protocol

This will be sent by the client to the server and from the server to the client every update (~500ms)

```ts
{
    // Only sent by Client
    "client": {
        "token": string?,
        "requests": [
            string,
            // "ALL_FILES",
        ]
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
