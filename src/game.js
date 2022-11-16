/**
 * game.js
 *
 * Contains interfacing logic with game wasm binary.
 * Also handles loading and predicting next input with neural network model.
 *
 */
import init, { Game } from './rs/pkg';
import * as tractjs from 'tractjs';
import { chunked, rotate, translate } from './utils';

export const initialize = async () => {
  self.width = 9;
  self.height = 9;

  // load game wasm
  console.log('loading wasm...');
  await init();
  self.game = new Game(width, height);

  // load onnx model
  console.log('loading onnx model...');
  const model = await tractjs.load('./snake.onnx', {
    inputFacts: {
      0: ['float32', [1, 81]],
    },
  });

  console.log('setting up...');
  self.score = () => self.game.score();

  self.reset = async () => {
    self.game.reset(self.width, self.height);
    return await self.game.tick();
  };

  self.tick = async () => {
    self.game.tick();
    return await self.state();
  };

  self.state = async () => {
    let state = chunked(self.game.state_js(), self.width):
    const cx = Math.floor(self.width / 2)
    const cy = Math.floor(self.height / 2)

    // view as neural network input
    const [q, _] = await self.calc_q();
    const [x, y, r] = self.game.state_params_js();

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

    const max_q = Math.max(...q).toFixed(2);

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
    return self.game.is_ended();
  };

  self.input = (cmd) => {
    self.game.input(cmd);
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
    const state = self.game.state_model_js();
    const pred = await model.predict([
      new tractjs.Tensor(new Float32Array(state), [1, 81]),
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
    self.game.input_turn(action);

    return q;
  };

  // player inputs
  self.up = () => self.game.input(0);
  self.down = () => self.game.input(1);
  self.left = () => self.game.input(2);
  self.right = () => self.game.input(3);

  self.set_food = (x, y) => {
    self.game.set_food(x, y);
  };

  // debugging
  // window.game = self.game

  return self;
};
