var node1 = { val: 1, next: null, prev: null };
var node2 = { val: 2, next: null, prev: null };
var node3 = { val: 3, next: null, prev: null };
var node4 = { val: 4, next: null, prev: null };
var node5 = { val: 5, next: null, prev: null };
var node6 = { val: 6, next: null, prev: null };
var node7 = { val: 7, next: null, prev: null };
var node8 = { val: 8, next: null, prev: null };
var node9 = { val: 9, next: null, prev: null };

node1.next = node2;
node2.next = node3;
node3.next = node4;
node4.next = node5;
node5.next = node6;
node6.next = node7;
node7.next = node8;
node8.next = node9;

node9.prev = node8;
node8.prev = node7;
node7.prev = node6;
node6.prev = node5;
node5.prev = node4;
node4.prev = node3;
node3.prev = node2;
node2.prev = node1;

var i = 1;
var iter = node1;
while (iter.next != null) {
  assert_eq(iter.val, i);
  iter = iter.next;
  i = i + 1;
}

var i = 9;
var iter = node9;
while (iter.prev != null) {
  assert_eq(iter.val, i);
  iter = iter.prev;
  i = i - 1;
}
