# Usage

```
curl --include \
     --no-buffer \
     --header "Connection: Upgrade" \
     --header "Upgrade: websocket" \
     --header "Host: localhost:3030" \
     --header "Origin: http://localhost:3030" \
     --header "Sec-WebSocket-Key: SGVsbG8sIHdvcmxkIQ==" \
     --header "Sec-WebSocket-Version: 13" \
     http://localhost:3030/ws
```

or if you have [websocat](https://github.com/vi/websocat) installed:

```
websocat ws:127.0.0.1:3030/ws
```

# References

- https://www.youtube.com/watch?v=feMKCAb-ZAg
