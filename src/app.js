import Vue from 'vue/dist/vue.js';
import { initialize } from './game.js';

initialize().then((game) => {
  let app = new Vue({
    el: '#app',

    data: {
      state: [],
      running: false,
      enable_ai: false,
      paused: false,
    },

    computed: {
      areaHeight: () => app.$el.clientWidth + 'px',
    },

    methods: {
      start: (enable_ai) => {
        app.enable_ai = enable_ai;

        // Reset game
        game.reset(app.width, app.height);
        app.width = game.width;
        app.height = game.height;

        // Set running
        app.running = true;

        // Focus hidden input field to receive key events
        app.$el.focus();

        // First tick
        app.tick();
      },

      stop: () => {
        console.log('stop game');
        app.running = false;
      },

      pause: () => {
        app.paused = !app.paused;
        if (!app.paused) {
          app.tick();
        }
      },

      // Setup ticker function
      tick: async () => {
        if (app.running) {
          app.state = game.tick();

          if (game.is_done()) {
            // halt for 1000 ms, then reset and continue
            setTimeout(() => {
              game.reset();
              app.tick();
            }, 1000);
            return;
          }

          if (app.enable_ai) {
            await game.input_ai();
          }

          if (!app.paused) {
            // Next tick
            setTimeout(app.tick, 100);
          }
        }
      },

      cell_clicked: (ev) => {
        // get cell x, y values
        const [x, y] = ev.target
          .getAttributeNode('item')
          .value.split(',')
          .map(Number);

        // set new food position
        game.snake.set_food(x, y);

        // get new state since food position is changed (unless invalid position)
        app.state = game.state();
      },
    },

    // runs on dom mount
    mounted: () => {
      console.log('created');
      // Setup keyEvents
      window.addEventListener('keydown', (e) => {
        if (!app.enable_ai) {
          if (e.keyCode == 37 || e.keyCode == 65)
            // left / a
            game.left();
          if (e.keyCode == 38 || e.keyCode == 87)
            // up / w
            game.up();
          if (e.keyCode == 39 || e.keyCode == 68)
            // right / d
            game.right();
          if (e.keyCode == 40 || e.keyCode == 83)
            // down / s
            game.down();
        }

        if (e.keyCode == 32) {
          // space / pause
          app.pause();
        }
      });

      // Mobile btn listeners
      document.getElementById('left-btn').addEventListener('mousedown', (e) => {
        if (!app.enable_ai) {
          game.down();
          game.left();
        }
      });
      document
        .getElementById('right-btn')
        .addEventListener('mousedown', (e) => {
          if (!app.enable_ai) {
            game.up();
            game.right();
          }
        });
    },
  });
});
