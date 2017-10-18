const wasm = require('./rs/src/main.rs')

wasm.initialize({noExitRuntime: true}).then(module => {
  // Create a Javascript wrapper around our Rust function
  const newGame = module.cwrap('new', 'Object')
  
  console.log('Calling rust functions from javascript!')
  console.log(newGame)
})
