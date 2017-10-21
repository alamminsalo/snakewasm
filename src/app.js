import Vue from 'vue/dist/vue.js';
import Game from './game.js';

var app = new Vue({
  el: '#app',

  data: {
    snake: [],
    food: [],
    height: 16,
    width: 16,
    running: false
  },

  methods: {
    start: () => { 
      app.running = true;
      Game.initialize().then((_game_) => {
        const game = _game_;

        // Setup ticker function
        app.tick = () => {
          game.tick();
          app.snake = game.snake.body();
          app.food = game.food();

          // Next tick
          setTimeout(app.tick, 100);
        };

        // Reset game
        game.reset(app.width, app.height);
        app.height = game.height();
        app.width = game.width();

        // First tick
        app.tick();
      });
    }
  }
})
