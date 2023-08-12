# TCPress **Experimental**

[![NPM version](https://img.shields.io/npm/v/tcpress-rs.svg?style=for-the-badge)](https://www.npmjs.com/package/tcpress-rs)

A web framework written in Rust for javascript runtime.

## Installation and Usage in NodeJs

```bash
npm install tcpress-rs
```
or
```bash
yarn add tcpress-rs
```

```ts
import { TCPress, Response, Request } from 'tcpress-rs';
import net from 'net'

const port = 7070;
const host = '0.0.0.0';
let app = new TCPress();
let sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
// app
app.get("/", [async (req: Request, res: Response, next: Function)=>{
    req.set("test", "app state 1");
    res.header("Powered-By", "TCPress")
    // await sleep(100);
    next();
}, async (req: Request, res: Response)=>{
    console.log("state test", req.get("test"));
    // console.log("headers", req.headers());
    // console.log("raw_body", req.body().raw_body());
    res.status(200).json({
        status: 200,
        message: "Hello World",
    });
}])

// uncaughtException
process.on('uncaughtException', function (err) {
    console.log(err);
});

// Tcpress + NodejsTCP
const server = net.createServer();
function sv(){
    server.listen(port, host, () => {
        console.log('TCP Server is running on port ' + port +'.');
    });
    server.on('connection', (sock) => {
        sock.on('data', (data) => {
            app.http(data, (res: Uint8Array)=>sock.write(res))
        });
    });
}
sv();
```

## Usage in Bun.sh
```ts
import { TCPress, Response, Request } from "tcpress-rs";

const port = 7070;
const host = '0.0.0.0';
let app = new TCPress();
let sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
// app
app.get("/", [async (req: Request, res: Response, next: Function) => {
    req.set("test", "app state 1");
    res.header("Powered-By", "TCPress")
    // await sleep(100);
    next();
}, async (req: Request, res: Response) => {
    console.log("state test", req.get("test"));
    // console.log("headers", req.headers());
    // console.log("raw_body", req.body().raw_body());
    res.status(200).json({
        status: 200,
        message: "Hello World",
    });
}])

// uncaughtException
process.on('uncaughtException', function (err) {
    console.log(err);
});
console.log('TCP Server is running on port ' + port +'.');
// Tcpress + BunTCP
Bun.listen({
    hostname: host,
    port: port,
    socket: {
        data(sock, data) {
            app.http(data, (res: Uint8Array) => sock.write(res))
        }
    }
});
```