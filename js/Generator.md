## 异步编程值 Generator

- 语法上，首先可以把它理解成，

  - Generator 函数是一个状态机，封装了多个内部状态。

执行 Generator 函数会返回一个遍历器对象，

- 所以还是一个遍历器对象生成函数。返回的遍历器对象，可以依次遍历 Generator 函数内部的每一个状态。

```
// function* Generator函数
function* helloWorldGenerator() {
  yield 'hello';
  yield 'world';
  return 'ending';
}

// Generator调用不立即执行，而是返回一个Iterator遍历器对象
var hw = helloWorldGenerator();
```

yield 暂停执行

```
function* gen() {
  yield  123 + 456; // yield后面的表达式123 + 456，不会立即求值，只会在next方法将指针移到这一句时，才会求值。
}
```
