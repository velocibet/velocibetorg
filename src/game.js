import init, { run_game } from '../botchi_game/pkg/botchi_game.js';

async function start() {
    await init();
    
    run_game();
}

start();