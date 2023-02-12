import init, {say_alert} from 'snake_game'
console.log('-------------- Module ------------')
init().then((_) => {
    console.log('-------------- WASM loaded ------------')
    say_alert('Test 1')
})