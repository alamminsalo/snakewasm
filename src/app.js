import Vue from 'vue/dist/vue.js';
import { initialize } from './game.js';

const setup_listeners = (game) => {
  // Setup keyEvents
  window.addEventListener('keydown', (e) => {
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
  });

  // Mobile btn listeners
  document.getElementById('left-btn').addEventListener('mousedown', (e) => {
    game.down();
    game.left();
  });
  document.getElementById('right-btn').addEventListener('mousedown', (e) => {
    game.up();
    game.right();
  });
};

initialize().then((game) => {
  let app = new Vue({
    el: '#app',

    data: {
      state: [],
      running: false,
    },

    computed: {
      areaHeight: () => app.$el.clientWidth + 'px',
    },

    methods: {
      start: (enable_ai) => {
        if (!enable_ai) {
          setup_listeners(game);
        }

        // Setup ticker function
        app.tick = async () => {
          app.state = game.tick();

          if (game.is_done()) {
            // halt for 1000 ms, then reset and continue
            setTimeout(() => {
              game.reset();
              app.tick();
            }, 1000);
            return;
          }

          if (enable_ai) {
            await game.input_ai();
          }

          // Next tick
          setTimeout(app.tick, 100);
        };

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
    },
  });
});
