"use strict";
let wasm_module;

function console_error(...args) {
  console.log(...args);
  Game.notify(args.join(" "));
}

module.exports.loop = function () {
  try {
    if (wasm_module && wasm_module.__wasm) {
      wasm_module.loop();
    } else {
      // attempt to load the wasm only if there's enough bucket to do a bunch of work this tick
      if (Game.cpu.bucket < 500) {
        console.log(
          "we are running out of time, pausing compile!" +
            JSON.stringify(Game.cpu)
        );
        return;
      }

      // replace this initialize function on the module
      wasm_module = require("screeps-world");
      // load the wasm instance!
      wasm_module.initialize_instance();
      // run the setup function, which configures logging
      wasm_module.setup();
      // go ahead and run the loop for its first tick
      wasm_module.loop();
      // stateScanner();
    }
  } catch (error) {
    console_error("caught exception:", error);
    if (error.stack) {
      console_error("stack trace:", error.stack);
    }
    console_error("resetting VM next tick.");
    wasm_module.__wasm = null;
  }
  clearMemory();
  exportStats();
};

// Call this function at the end of your main loop

function exportStats() {
  // Reset stats object
  Memory.stats = {
    gcl: {},
    rooms: {},
    cpu: {},
  };

  Memory.stats.time = Game.time;

  // Collect room stats
  for (let roomName in Game.rooms) {
    let room = Game.rooms[roomName];
    let isMyRoom = room.controller ? room.controller.my : false;
    if (isMyRoom) {
      let roomStats = (Memory.stats.rooms[roomName] = {});
      roomStats.storageEnergy = room.storage ? room.storage.store.energy : 0;
      roomStats.terminalEnergy = room.terminal ? room.terminal.store.energy : 0;
      roomStats.energyAvailable = room.energyAvailable;
      roomStats.energyCapacityAvailable = room.energyCapacityAvailable;
      roomStats.controllerProgress = room.controller.progress;
      roomStats.controllerProgressTotal = room.controller.progressTotal;
      roomStats.controllerLevel = room.controller.level;
    }
  }

  // Collect GCL stats
  Memory.stats.gcl.progress = Game.gcl.progress;
  Memory.stats.gcl.progressTotal = Game.gcl.progressTotal;
  Memory.stats.gcl.level = Game.gcl.level;

  // Collect CPU stats
  Memory.stats.cpu.bucket = Game.cpu.bucket;
  Memory.stats.cpu.limit = Game.cpu.limit;
  Memory.stats.cpu.used = Game.cpu.getUsed();
}

// const stateScanner = function () {
//   // 每 20 tick 运行一次
//   if (Game.time % 10) return;

//   if (!Memory.stats) Memory.stats = {};

//   // 统计 GCL / GPL 的升级百分比和等级
//   Memory.stats.gcl = (Game.gcl.progress / Game.gcl.progressTotal) * 100;
//   Memory.stats.gclLevel = Game.gcl.level;
//   Memory.stats.gpl = (Game.gpl.progress / Game.gpl.progressTotal) * 100;
//   Memory.stats.gplLevel = Game.gpl.level;
//   // CPU 的当前使用量
//   Memory.stats.cpu = Game.cpu.getUsed();
//   // bucket 当前剩余量
//   Memory.stats.bucket = Game.cpu.bucket;
// };

const clearMemory = function () {
  if (Game.time % 100) return;

  for (var i in Memory.creeps) {
    if (!Game.creeps[i]) {
      delete Memory.creeps[i];
    }
  }
};
