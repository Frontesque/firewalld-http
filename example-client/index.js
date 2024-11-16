const http = require('http');

async function post({ body, ...opt }) {
    return new Promise((accept, reject) => {
        const data = JSON.stringify(body);
        console.log(data);
        const options = {
            method: 'POST',
            headers: {
                'Content-Type': 'multipart/form-data',
                'Content-Length': data.length
            },
            ...opt
        };
        const req = http.request(options, (res) => {
            let data = '';
            res.on('data', (chunk) => {
                data += chunk;
            });
            res.on('end', () => {
                accept(data);
            });
        }).on("error", (err) => {
            reject(err);
        });
        req.write(data);
        req.end();
    })
}

async function request(body) {
    return await post({
        hostname: 'localhost',
        port: 9988,
        path: '/',
        body: body
    });
}

async function main() {
    const res = await request({
        first_name: 'Fred',
        last_name: 'Flintstone'
    });
    console.log(res);
}
main();