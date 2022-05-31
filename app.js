const lol = require('./index.node')

function fibonacci(num) {
    var a = 1,
      b = 0,
      temp;
    while (num >= 0) {
      temp = a;
      a = a + b;
      b = temp;
      num--;
    }
    return b;
  }

// console.log(lol.somaFoo(10,5));
// console.log(lol.hello("Pietro"));

a = []
for (let i = 1; i< 70; i++){
    a.push(i)
}
//console.log(a);
let start = {};
start = performance.now();
// a.forEach(v => fibonacci(v))
console.log(fibonacci(70));
console.log((performance.now() - start) );

start = performance.now();
console.log(lol.fibonaci(70));
// a.forEach(v => lol.fibonaci(v))
console.log((performance.now() - start) );
