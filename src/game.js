import init, { state_js, state_model_js } from './rs/pkg';
import * as tractjs from 'tractjs';

export const initialize = async () => {
  console.log('initialize game...');

  console.log('loading wasm...');
  const snake = await init();

  console.log(snake.reset(9, 9));
  snake.tick();

  console.log(snake.is_ended());

  window.snake = snake;
  window.snake_state = state_js;
  window.snake_state_model = state_model_js;

  //console.log('loading ONNX graph...');
  //const model = await tractjs.load('./onnx/snake.onnx');
};
