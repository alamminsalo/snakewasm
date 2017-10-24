import Vue from 'vue/dist/vue.js';
import Game from './game.js';
import _ from 'lodash';

let app = new Vue({
  el: '#app',

  data: {
    snake: { x: {}, y: {}},
    food: {},
    height: 16,
    width: 16,
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

    isSnake: (x,y) => _.some(app.snake, {'x': x, 'y': y}),
    isFood: (x,y) => app.food.x == x && app.food.y == y,

    start: () => {
      Game.initialize().then((_game_) => {
        const game = _game_;

        window.GAME = game;

        app.up = () => game.snake.up();
        app.down = () => game.snake.down();
        app.left = () => game.snake.left();
        app.right = () => game.snake.right();

        // Setup keyEvents
        window.addEventListener('keydown', (e) => {
          if (e.keyCode == 37 || e.keyCode == 65) // left / a
            app.left();
          if (e.keyCode == 38 || e.keyCode == 87) // up / w
            app.up();
          if (e.keyCode == 39 || e.keyCode == 68) // right / d
            app.right();
          if (e.keyCode == 40 || e.keyCode == 83) // down / s
            app.down();
        });

        // Mobile btn listeners
        document.getElementById('left-btn').addEventListener('mousedown', (e) => {
          app.down();
          app.left();
        });
        document.getElementById('right-btn').addEventListener('mousedown', (e) => {
          app.up();
          app.right();
        });

        // Setup ticker function
        app.tick = () => {
          game.tick();

          if (game.isEnded())
            game.reset(app.width, app.height);

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

