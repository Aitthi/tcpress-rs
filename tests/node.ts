import { TCPress, Response, Request } from './../pkg/tcpress_rs';
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
    let  tc = 0
    server.on('connection', (sock) => {
        sock.on('data', (data) => {
            tc++
            let tm_lable = `lap_${tc}`
            console.time(tm_lable)
            app.http(data, (res: Uint8Array)=>{
                sock.write(res);
                console.timeEnd(tm_lable)
            })
        });
    });
}
sv();