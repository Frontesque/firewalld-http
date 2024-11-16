const axios = require('axios');

async function main() {
    const response = await axios.get('http://localhost:9988/');
    console.log(response.data);
}
main();