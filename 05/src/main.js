class Ship {
  stacks = [];

  move(count, from, to) {
    let from_stack = this.stacks[from];
    let to_stack = this.stacks[to];
    if (count > from_stack.length) {
      throw new Error(
        `Not enough from stack: move ${count} from ${from + 1} to ${to + 1}`
      );
    }
    let move = from_stack.slice(from_stack.length - count);
    to_stack.push(...move);
    from_stack.length -= count;

    // let q = count;
    // while (q > 0) {
    //   if (from_stack.length === 0) {
    //     throw new Error(
    //       `Empty from stack: move ${count} from ${from + 1} to ${to + 1}`
    //     );
    //   }
    //   let top = from_stack.pop();
    //   to_stack.push(top);
    //   q -= 1;
    // }
  }

  tops() {
    const tops = [];
    for (const stack of this.stacks) {
      tops.push(stack.at(-1) ?? "");
    }
    return tops.join("");
  }

  operate(moves) {
    moves = moves.split("\n");
    for (let i = 0; i < moves.length; i++) {
      const move = moves[i];
      const { count, from, to } = move.match(
        /move (?<count>\d+) from (?<from>\d+) to (?<to>\d+)/
      ).groups;
      this.move(+count, from - 1, to - 1);
      // console.log(this.tops());
    }
  }
}

function make_ship(str) {
  let stacks = str.split("\n").reverse();
  const ship = new Ship();

  const length = Number(stacks[0].trim().split(/\s+/).pop());
  for (let i = 0; i < length; i++) {
    ship.stacks[i] = [];
  }

  for (const stack of stacks.slice(1)) {
    const crates = stack.split(" ");
    let i = 0;
    for (const crate of crates) {
      const id = crate.match(/\[(?<id>[A-Z])\]/)?.groups?.id;
      if (id) {
        ship.stacks[i].push(id);
      }
      i++;
    }
  }

  return ship;
}

const test_stacks = `[_] [D] [_]
[N] [C] [_]  
[Z] [M] [P]
 1   2   3 `;

const test_moves = `move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2`;

const test_ship = make_ship(test_stacks);
console.log(test_ship.tops());
test_ship.operate(test_moves);
console.log(test_ship.tops());

const data = require("./data");

const data_ship = make_ship(data.stacks);
console.log(data_ship.tops());
data_ship.operate(data.moves);
console.log(data_ship.tops());
