import Vue from 'vue/dist/vue.js';
import Game from './game.js';
import _ from 'lodash';

let app = new Vue({
  el: '#app',

  data: {
    snake: { x: {}, y: {}},
    food: [],
    height: 24,
    width: 24,
    running: false
  },

  computed: {
    areaHeight: () => app.$el.clientWidth + 'px'
  },

  methods: {
    up: () => {},
    down: () => {},
    left: () => {},
    right: () => {},

    start: () => {
      Game.initialize().then((_game_) => {
        const game = _game_;

        app.up = () => game.snake.up();
        app.down = () => game.snake.down();
        app.left = () => game.snake.left();
        app.right = () => game.snake.right();

        // Setup keyEvents
        window.addEventListener('keydown', (e) => {
          if (e.keyCode == 37)
            app.left();
          if (e.keyCode == 38)
            app.up();
          if (e.keyCode == 39)
            app.right();
          if (e.keyCode == 40)
            app.down();
        });

        // Setup ticker function
        app.tick = () => {
          game.tick();

          // Refresh snake
          app.snake = game.snake.body();


          // Refresh food
          app.food = game.food();

          // Next tick
          setTimeout(app.tick, 100);
        };

        // Reset game
        game.reset(app.width, app.height);
        app.height = game.height();
        app.width = game.width();

        // Set running
        app.running = true;

        // Focus hidden input field to receive key events
        app.$el.focus();


        // First tick
        app.tick();
      });
    }
  }
})

