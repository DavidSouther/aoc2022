const test_monkeys = () => [
  {
    items: [79, 98],
    op: (old) => old * 19,
    test: [23, 2, 3],
    inspected: 0,
  },
  {
    items: [54, 65, 75, 74],
    op: (old) => old + 6,
    test: [19, 2, 0],
    inspected: 0,
  },
  {
    items: [79, 60, 97],
    op: (old) => old * old,
    test: [13, 1, 3],
    inspected: 0,
  },
  {
    items: [74],
    op: (old) => old + 3,
    test: [17, 0, 1],
    inspected: 0,
  },
];

const data_monkeys = () => [
  {
    items: [96, 60, 68, 91, 83, 57, 85],
    op: (o) => o * 2,
    test: [17, 2, 5],
    inspected: 0,
  },
  {
    items: [75, 78, 68, 81, 73, 99],
    op: (o) => o + 3,
    test: [13, 7, 4],
    inspected: 0,
  },
  {
    items: [69, 86, 67, 55, 96, 69, 94, 85],
    op: (o) => o + 6,
    test: [19, 6, 5],
    inspected: 0,
  },
  {
    items: [88, 75, 74, 98, 80],
    op: (old) => old + 5,
    test: [7, 7, 1],
    inspected: 0,
  },
  {
    items: [82],
    op: (old) => old + 8,
    test: [11, 0, 2],
    inspected: 0,
  },
  {
    items: [72, 92, 92],
    op: (old) => old * 5,
    test: [3, 6, 3],
    inspected: 0,
  },
  {
    items: [74, 61],
    op: (old) => old * old,
    test: [2, 3, 1],
    inspected: 0,
  },
  {
    items: [76, 86, 83, 55],
    op: (old) => old + 4,
    test: [5, 4, 0],
    inspected: 0,
  },
];

const test_monkeys_n = () => [
  {
    items: [79n, 98n],
    op: (old) => old * 19n,
    test: [23n, 2, 3],
    inspected: 0,
  },
  {
    items: [54n, 65n, 75n, 74n],
    op: (old) => old + 6n,
    test: [19n, 2, 0],
    inspected: 0,
  },
  {
    items: [79n, 60n, 97n],
    op: (old) => old * old,
    test: [13n, 1, 3],
    inspected: 0,
  },
  {
    items: [74n],
    op: (old) => old + 3n,
    test: [17n, 0, 1],
    inspected: 0,
  },
];

const data_monkeys_n = () => [
  {
    items: [96n, 60n, 68n, 91n, 83n, 57n, 85n],
    op: (o) => o * 2n,
    test: [17n, 2, 5],
    inspected: 0,
  },
  {
    items: [75n, 78n, 68n, 81n, 73n, 99n],
    op: (o) => o + 3n,
    test: [13n, 7, 4],
    inspected: 0,
  },
  {
    items: [69n, 86n, 67n, 55n, 96n, 69n, 94n, 85n],
    op: (o) => o + 6n,
    test: [19n, 6, 5],
    inspected: 0,
  },
  {
    items: [88n, 75n, 74n, 98n, 80n],
    op: (old) => old + 5n,
    test: [7n, 7, 1],
    inspected: 0,
  },
  {
    items: [82n],
    op: (old) => old + 8n,
    test: [11n, 0, 2],
    inspected: 0,
  },
  {
    items: [72n, 92n, 92n],
    op: (old) => old * 5n,
    test: [3n, 6, 3],
    inspected: 0,
  },
  {
    items: [74n, 61n],
    op: (old) => old * old,
    test: [2n, 3, 1],
    inspected: 0,
  },
  {
    items: [76n, 86n, 83n, 55n],
    op: (old) => old + 4n,
    test: [5n, 4, 0],
    inspected: 0,
  },
];

// After each monkey inspects an item but before it tests your worry level, your relief that the monkey's inspection didn't damage the item causes your worry level to be divided by three and rounded down to the nearest integer.

// The monkeys take turns inspecting and throwing items. On a single monkey's turn, it inspects and throws all of the items it is holding one at a time and in the order listed. Monkey 0 goes first, then monkey 1, and so on until each monkey has had one turn. The process of each monkey taking a single turn is called a round.

// When a monkey throws an item to another monkey, the item goes on the end of the recipient monkey's list. A monkey that starts a round with no items could end up inspecting and throwing many items by the time its turn comes around. If a monkey is holding no items at the start of its turn, its turn ends.

// Monkey inspects an item with a worry level of 79.
//   Worry level is multiplied by 19 to 1501.
//   Monkey gets bored with item. Worry level is divided by 3 to 500.
//   Current worry level is not divisible by 23.
//   Item with worry level 500 is thrown to monkey 3.
function round(monkeys, reduce) {
  const lcd = monkeys.reduce((a, m) => m.test[0] * a, 1);

  for (const monkey of monkeys) {
    while (monkey.items.length > 0) {
      let item = monkey.items.shift();
      monkey.inspected += 1;
      item = monkey.op(item);
      if (reduce) {
        item = Math.floor(item / 3);
      } else {
        item = item % lcd;
      }
      const [d, t, f] = monkey.test;
      let next = item % d == 0n ? t : f;
      monkeys[next].items.push(item);
    }
  }
}

function rounds(monkeys, ct, reduce) {
  monkeys.forEach((m) => (m.inspected = 0));

  for (let i = 0; i < ct; i++) {
    round(monkeys, reduce);
  }

  monkeys.sort((a, b) => b.inspected - a.inspected);

  console.log(monkeys[0].inspected * monkeys[1].inspected);
}

rounds(test_monkeys(), 20, true);
rounds(data_monkeys(), 20, true);

rounds(test_monkeys(), 10_000, false);
rounds(data_monkeys(), 10_000, false);
