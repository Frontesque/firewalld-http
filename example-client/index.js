const axios = require('axios');

async function main() {
    const response = await axios.post('http://localhost:9988/', {
        firstName: 'Fred',
        lastName: 'Flintstone'
    }, {
        headers: {
          'Content-Type': 'multipart/form-data'
        }
    });
    console.log(response.data);
}
main();