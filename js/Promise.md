// Promise::
// 1. 对象的状态不受外界影响。
// 2. 一旦状态改变，就不会再变， (事件的特点是，如果你错过了它，再去监听，是得不到结果的。)

// 缺点:
// 1. 无法取消Promise，**一旦新建它就会立即执行**，无法中途取消。
// 2. 如果不设置回调函数，Promise内部抛出的错误，不会反应到外部。
// 3. 当处于pending状态时，无法得知目前进展到哪一个阶段（刚刚开始还是即将完成）。

// 一. ** 新建后立即执行** ;
;```
let promise = new Promise(function(resolve, reject) {
  console.log('Promise');
  resolve();
});

promise.then(function() {
  console.log('resolved.');
});

console.log('Hi!');

// Promise
// Hi!
// resolved
```
// 二、Promise 内部resolve/reject() 以后还会继续执行
// 1. resolved之后的错误(throw error) 是无效的、不会被外部捕获; 因为 Promise 的状态一旦改变，就永久保持该状态，不会再变了。
;```
const promise = new Promise(function(resolve, reject) {
  resolve('ok');
  throw new Error('test');
});
promise
  .then(function(value) { console.log(value) })
  .catch(function(error) { console.log(error) });
// ok
```

// 2. 立即 resolved 的 Promise(后的代码) 是在本轮事件循环的末尾执行，总是晚于本轮循环的同步任务。
// 3. 一般来说resolved或者reject以后就不要添加代码了, 而应放到.then() catch里面```
;```
new Promise((resolve, reject) => {
  resolve(1); // ==> 改为return resolve(1);
  console.log(2);
}).then(r => {
  console.log(r);
});
// 2
// 1
```

// Promise返回Promise, 内层的Promise状态决定了 外层的状态
;```
```
