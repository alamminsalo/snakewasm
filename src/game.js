/**
 * game.js
 *
 * Contains interfacing logic with game wasm binary.
 * Also handles loading and predicting next input with neural network model.
 *
 */
import init, { state_js, state_model_js, state_params_js } from './rs/pkg';
import * as tractjs from 'tractjs';

// chunks array to n parts
const chunked = (arr, n) =>
  Array.from({ length: Math.ceil(arr.length / n) }, (_, i) =>
    arr.slice(i * n, i * n + n),
  );

// rotates array n times clockwise
const rotate = (arr, n = 1) => {
  const x = Math.floor(arr.length/ 2);
  const y = arr.length - 1;
  while (n-- > 0) {
    for (let i = 0; i < x; i++) {
       for (let j = i; j < y - i; j++) {
          k = arr[i][j];
          arr[i][j] = arr[y - j][i];
          arr[y - j][i] = arr[y - i][y - j];
          arr[y - i][y - j] = arr[j][y - i]
          arr[j][y - i] = k
       }
    }
  }
  return arr
}

// console.log(
//   rotate(
//     [
//       [1,1,1],
//       [2,2,2],
//       [3,3,3]
//     ],
//     1
//   ),
//     [
//       [3,2,1],
//       [3,2,1],
//       [3,2,1],
//     ],
// );
// 
// console.log(
//   rotate(
//     [
//       [1,1,1],
//       [2,2,2],
//       [3,3,3]
//     ],
//     3
//   ),
//     [
//       [1,2,3],
//       [1,2,3],
//       [1,2,3],
//     ],
// );

// pans input 2d array by offset x, y
const translate = (a, x, y) => {
  // y-axis
  let b = a.splice(y * -1)
  a = b.concat(a)
  a = rotate(a, 1);

  // x-axis
  b = a.splice(x * -1);
  a = b.concat(a)
  a = rotate(a, 3);

  return a;
}

//console.log(
//  'test positive translate',
//  translate(
//    [
//      [1,2,3],
//      [4,5,6],
//      [7,8,9]
//    ],
//    0, 1
//  ),
//);
//
//console.log(
//  'test negative translate',
//  translate(
//    [
//      [1,2,3],
//      [4,5,6],
//      [7,8,9]
//    ],
//    0, -1
//  ),
//);

export const initialize = async () => {
  self.width = 9;
  self.height = 9;

  // load game wasm
  const snake = await init();

  // load onnx model
  const model = await tractjs.load('./snake.onnx', {
    inputFacts: {
      0: ['float32', [1, 9, 9, 1]],
    },
  });

  self.score = () => {
    return snake.score();
  };

  self.reset = () => {
    snake.reset(self.width, self.height);
    return self.tick();
  };

  self.tick = async () => {
    snake.tick();
    return await self.state();
  };

  self.state = async () => {
    let state = chunked(state_js(), self.width):
    const cx = Math.floor(self.width / 2)
    const cy = Math.floor(self.height / 2)

    // view as neural network input
    const [q, _] = await self.calc_q();
    const [x, y, r] = state_params_js();

    // translate state to center
    state = translate(state, cx - x, cy - y);
    state = rotate(state, 4 - r);

    const cell_q_value = (idx) => {
      switch (idx) {
          // forward
        case 31:
          return q[0].toFixed(2);
        case 39:
          return q[1].toFixed(2);
        case 41:
          return q[2].toFixed(2);
      }
      return null;
    };

    const max_q = Math.max(...q);

    state = chunked(state.flat().map((c, idx) => ({
      v: c,
      q: cell_q_value(idx),
      q_is_max: cell_q_value(idx) == max_q,
    })), self.width);

    // translate and roll back to human-perspective
    state = translate(rotate(state, r), x - cx, y - cx);

    return state;
  };

  self.is_done = () => {
    return snake.is_ended();
  };

  self.input = (cmd) => {
    snake.input(cmd);
  };

  // toggles between viewing game as player or nn perspective
  let view_nn = false;
  self.toggle_view_nn = () => {
    view_nn = !view_nn;
    return view_nn;
  };

  // calculates q values for current state
  self.calc_q = async () => {
    // get current model state
    const state = state_model_js();
    const pred = await model.predict([
      new tractjs.Tensor(new Float32Array(state), [1, 9, 9, 1]),
    ]);

    // get index of maximum predicted Q-value
    const q = pred[0].data;

    return [q, state];
  };

  // uses ai-prediction for next input when called
  self.input_ai = async () => {
    const [q, _state] = await self.calc_q();
    const max_q = Math.max(...q);
    const action = q.indexOf(max_q);

    // pass action as input_turn param to game
    snake.input_turn(action);

    return q;
  };

  // player inputs
  self.up = () => snake.input(0);
  self.down = () => snake.input(1);
  self.left = () => snake.input(2);
  self.right = () => snake.input(3);

  self.set_food = (x, y) => {
    snake.set_food(x, y);
  };

  // debugging
  // window.snake = snake;

  return self;
};
