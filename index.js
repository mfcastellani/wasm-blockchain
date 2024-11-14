import init, { validate } from './pkg/blockchain_test.js';

async function run() {
    await init();
    validate();
}

run();