import { TCPress, Response, Request } from "tcpress-rs";

const port = 7070;
const host = '0.0.0.0';
let app = new TCPress();
let sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
// app
app.get("/", [async (req: Request, res: Response) => {
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
let  tc = 0
// Tcpress + BunTCP
Bun.listen({
    hostname: host,
    port: port,
    socket: {
        data(sock, data) {
            tc++
            let tm_lable = `lap_${tc}`
            console.time(tm_lable)
            app.http(data, (res: Uint8Array) => {
                sock.write(res);
                console.timeEnd(tm_lable)
            })
        }
    }
});