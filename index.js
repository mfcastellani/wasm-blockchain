import init, { validate } from './pkg/blockchain_test.js';

async function run() {
    await init();
    console.log(validate());
}

run();