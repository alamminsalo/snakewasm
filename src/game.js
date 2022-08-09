/**
 * game.js
 *
 * Contains interfacing logic with game wasm binary.
 * Also handles loading and predicting next input with neural network model.
 *
 */
import init, { state_js, state_model_js } from './rs/pkg';
import * as tractjs from 'tractjs';

// chunks array to n parts
const chunked = (arr, n) =>
  Array.from({ length: Math.ceil(arr.length / n) }, (_, i) =>
    arr.slice(i * n, i * n + n),
  );

export const initialize = async () => {
  self.width = 9;
  self.height = 9;

  // load game wasm
  self.snake = await init();

  // load onnx model
  const model = await tractjs.load('./snake.onnx', {
    inputFacts: {
      0: ['float32', [1, 9, 9, 1]],
    },
  });

  self.reset = () => {
    self.snake.reset(self.width, self.height);
    return self.tick();
  };

  self.tick = () => {
    self.snake.tick();
    return self.state();
  };

  self.state = () => {
    const state = state_js();
    return chunked(state, self.width);
  };

  self.is_done = () => {
    return self.snake.is_ended();
  };

  self.input = (cmd) => {
    self.snake.input(cmd);
  };

  // uses ai-prediction for next input when called
  self.input_ai = async () => {
    // get current model state
    const state = state_model_js();
    const pred = await model.predict([
      new tractjs.Tensor(new Float32Array(state), [1, 9, 9, 1]),
    ]);

    // get index of maximum predicted Q-value
    const q = pred[0].data;
    const max_q = Math.max(...q);
    const action = q.indexOf(max_q);

    // pass action as input_turn param to game
    self.snake.input_turn(action);
  };

  // player inputs
  self.up = () => self.snake.input(0);
  self.down = () => self.snake.input(1);
  self.left = () => self.snake.input(2);
  self.right = () => self.snake.input(3);

  // debugging
  // window.game = self;

  return self;
};
