//const wasm = require('./rs/src/main.rs');
const wasm = await WebAssembly.instantiateStreaming(
  fetch('src/rs/snake_bg.wasm'),
);

var Game = {
  initialize: () =>
    wasm.initialize({ noExitRuntime: true }).then((module) => {
      // Create a Javascript wrapper around our Rust function
      return {
        height: module.cwrap('game_height', 'number', []),
        width: module.cwrap('game_width', 'number', []),
        tick: module.cwrap('tick', null, []),
        reset: module.cwrap('reset', null, []),
        isEnded: module.cwrap('is_ended', 'boolean', []),

        snake: {
          up: module.cwrap('snake_up', null, []),
          down: module.cwrap('snake_down', null, []),
          left: module.cwrap('snake_left', null, []),
          right: module.cwrap('snake_right', null, []),

          // Returns snake as [{x,y}]
          body: (function () {
            const len = module.cwrap('snake_len', 'number', []);
            const xat = module.cwrap('snake_x_at', 'number', ['number']);
            const yat = module.cwrap('snake_y_at', 'number', ['number']);
            return function () {
              let arr = [];
              let snake_len = len();
              for (var i = 0; i < snake_len; i++) {
                arr.push({ x: xat(i), y: yat(i) });
              }
              return arr;
            };
          })(),
        },

        food: (function () {
          const fx = module.cwrap('food_x', 'number', []);
          const fy = module.cwrap('food_y', 'number', []);
          return function () {
            return {
              x: fx(),
              y: fy(),
            };
          };
        })(),
      };
    }),
};

export default Game;
