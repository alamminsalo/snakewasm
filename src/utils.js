
// chunks array to n parts
export const chunked = (arr, n) =>
  Array.from({ length: Math.ceil(arr.length / n) }, (_, i) =>
    arr.slice(i * n, i * n + n),
  );

// rotates array n times clockwise
export const rotate = (arr, n = 1) => {
  const x = Math.floor(arr.length/ 2);
  const y = arr.length - 1;
  while (n-- > 0) {
    for (let i = 0; i < x; i++) {
       for (let j = i; j < y - i; j++) {
          const k = arr[i][j];
          arr[i][j] = arr[y - j][i];
          arr[y - j][i] = arr[y - i][y - j];
          arr[y - i][y - j] = arr[j][y - i];
          arr[j][y - i] = k;
       }
    }
  }
  return arr;
}

// console.log(
//   rotate(
//     [
//       [1,1,1],
//       [2,2,2],
//       [3,3,3]
//     ],
//     1
//   ),
//     [
//       [3,2,1],
//       [3,2,1],
//       [3,2,1],
//     ],
// );
// 
// console.log(
//   rotate(
//     [
//       [1,1,1],
//       [2,2,2],
//       [3,3,3]
//     ],
//     3
//   ),
//     [
//       [1,2,3],
//       [1,2,3],
//       [1,2,3],
//     ],
// );

// pans input 2d array by offset x, y
export const translate = (a, x, y) => {
  // y-axis
  let b = a.splice(y * -1);
  a = b.concat(a);
  a = rotate(a, 1);

  // x-axis
  b = a.splice(x * -1);
  a = b.concat(a)
  a = rotate(a, 3);

  return a;
}

//console.log(
//  'test positive translate',
//  translate(
//    [
//      [1,2,3],
//      [4,5,6],
//      [7,8,9]
//    ],
//    0, 1
//  ),
//);
//
//console.log(
//  'test negative translate',
//  translate(
//    [
//      [1,2,3],
//      [4,5,6],
//      [7,8,9]
//    ],
//    0, -1
//  ),
//);

