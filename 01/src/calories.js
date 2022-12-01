const EventEmitter = require("events");

const elves = new EventEmitter();

let calories = [];
elves.on("elf", (elf) => {
  const meals = elf.split("\n").map(Number);
  const cals = meals.reduce((a, b) => a + b, 0);
  if (cals !== undefined && !isNaN(cals)) {
    calories.push(cals);
  }
});
elves.on("end", () => {
  calories.sort((a, b) => b - a);
  console.log(calories[0] + calories[1] + calories[2]);
});

process.stdin.on("data", (data) => {
  `${data}`.split("\n\n").forEach((elf) => elves.emit("elf", elf));
});

process.stdin.on("end", () => {
  elves.emit("end");
});
