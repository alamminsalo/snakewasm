/**
 *
 * app.js
 *
 * Contains vue component, user inputs
 * and game ticking logic.
 *
 */
import Vue from 'vue';
import { initialize } from './game.js';
  
initialize().then((game) => {
  let app = new Vue({
    el: '#app',

    data: {
      state: [],
      running: false,
      enable_ai: false,
      paused: false,
      autopause: false, // when enabled, automatically pauses game on receiving score point
      score: 0,
      show_q: false,
      speed: 1.0,
    },

    computed: {
      areaHeight: () => app.$el.clientWidth + 'px',
    },

    methods: {
      // starts the game
      start: async (enable_ai) => {
        app.enable_ai = enable_ai;

        // disable autopause if human is playing
        if (!enable_ai) {
          app.autopause = false;
        }

        // Set running
        app.running = true;

        // Focus hidden input field to receive key events
        app.$el.focus();

        // reset score
        app.score = 0;

        // Reset game and receive first state
        app.state = await game.reset();

        // First tick
        app.tick();
      },

      stop: () => {
        app.running = false;
      },

      // Setup ticker function
      tick: async () => {
        if (app.running && !app.paused) {
          // if enabled, get input from ai model
          if (app.enable_ai) 
            await game.input_ai();

          // tick game and receive next
          app.state = await game.tick();

          // if game has not ended, schedule next tick in 100 ms
          if (!game.is_done()) setTimeout(app.tick, 100 / app.speed);
          // otherwise, restart the game after 1 second
          else setTimeout(() => app.start(app.enable_ai), 1000);
        }

        // update score and handle autopausing if enabled
        const new_score = game.score();
        if (app.autopause && new_score > app.score) {
          app.paused = true;
        }
        app.score = new_score;
      },

      cell_clicked: async (ev) => {
        // get cell x, y values
        const [x, y] = ev.target
          .getAttributeNode('item')
          .value.split(',')
          .map(Number);

        // set new food position
        game.set_food(x, y);

        // get new state since food position is changed (unless invalid position)
        app.state = await game.state();

        // if autopause, unpause game
        if (app.autopause && app.paused) {
          app.toggle_pause();
        }
      },

      toggle_pause: () => {
        app.paused = !app.paused;

        // when unpausing, continue ticking
        if (!app.paused) app.tick();
      },

      toggle_autopause: (ev) => {
        app.autopause = !app.autopause;

        // if disabling autopause, unpause game
        if (!app.autopause && app.paused) {
          app.toggle_pause();
        }

        // Prevent the button from receiving focus,
        // because interesting stuff starts to happen
        // when space both toggles autopause and pause simultaneously.
        ev.target.blur();
      },

      toggle_show_q: (ev) => {
        app.show_q = !app.show_q;
      },

      set_speed: (speed) => {
        app.speed = speed;
      },
    },

    // runs on dom mount
    mounted: () => {
      // Setup keyEvents
      window.addEventListener('keydown', (e) => {
        if (!app.enable_ai) {
          // left / a
          if (e.keyCode == 37 || e.keyCode == 65) game.left();
          // up / w
          if (e.keyCode == 38 || e.keyCode == 87) game.up();
          // right / d
          if (e.keyCode == 39 || e.keyCode == 68) game.right();
          // down / s
          if (e.keyCode == 40 || e.keyCode == 83) game.down();
        }

        // space pauses the game
        if (e.keyCode == 32) app.toggle_pause();
      });

      // Mobile btn listeners
      document.getElementById('left-btn').addEventListener('mousedown', (e) => {
        if (!app.enable_ai) {
          game.up();
          game.left();
        }
      });
      document
        .getElementById('right-btn')
        .addEventListener('mousedown', (e) => {
          if (!app.enable_ai) {
            game.down();
            game.right();
          }
        });
    },
  });
});
