function find_max(array, size) {
  let i = 0;
  let max = array[0];
  while (i < size) {
    if (max < array[i]) {
      max = array[i];
    }
    i = i + 1;
  }
  return max;
}

function find_min(array, size) {
  let i = 0;
  let min = array[0];
  while (i < size) {
    if (min > array[i]) {
      min = array[i];
    }
    i = i + 1;
  }
  return min;
}

function buble_sort(array, size) {
  let i = 0;
  while (i < size - 1) {
    let j = 0;
    while (j < size - i - 1) {
      if (array[j] > array[j + 1]) {
        var tmp = array[j];
        array[j] = array[j + 1];
        array[j + 1] = tmp;
      }
      j = j + 1;
    }
    i = i + 1;
  }
  return array;
}

function assert_eq_array(array1, size1, array2, size2) {
  assert(size1 == size2);
  let i = 0;
  while (i < size1) {
    i = i + 1;
    assert_eq(array1[i], array2[i]);
  }
}

let array = [2, 3, 1, 100, 2, 23, 12, 8];
let size = 8;

assert_eq(find_max(array, size), 100);
assert_eq(find_min(array, size), 1);
assert_eq_array(
  buble_sort(array, size),
  size,
  [1, 2, 2, 3, 8, 12, 23, 100],
  size
);
