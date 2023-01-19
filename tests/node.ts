import { TCPress, Response, Request } from './../pkg/tcpress_rs';
import net from 'net'

const port = 7070;
const host = '0.0.0.0';
let app = new TCPress();
let sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
// app
app.get("/", [async (req: Request, res: Response)=>{
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
            app.http(data, (res: Uint8Array)=>{
                sock.write(res);
            })
        });
    });
}
sv();