import { Observable, range } from "https://esm.sh/rxjs@~7.5.0";
import { filter, map } from "https://esm.sh/rxjs@~7.5.0/operators";

range(1, 20)
  .pipe(
    filter((x: number) => x % 2 === 1),
    map((x: number) => x + x),
  )
  .subscribe((x: number) => console.log(`Number is: ${x}`));

const observable = new Observable((subscriber) => {
  subscriber.next(1);
  subscriber.next(2);
  subscriber.next(3);
  setTimeout(() => {
    subscriber.next(4);
    subscriber.complete();
  }, 1000);
});

console.log("Just before subscribe");
const subscription = observable.subscribe({
  next(x) {
    console.log(`Got value: ${x}`);
  },
  error(err) {
    console.error("Rip", err);
  },
  complete() {
    console.log("Doneskies");
  },
});
console.log("Just after subscribe");
subscription.unsubscribe();

const fibonacciObservable = new Observable((subscriber) => {
  let prev = 0;
  let next = 1;
  const fib = () => {
    subscriber.next(prev);
    const tmp = prev + next;
    prev = next;
    next = tmp;

    if (next === Infinity) {
        subscriber.complete();
    }
  };
  const handle = setInterval(fib, 10);
  
  return () => clearInterval(handle);
});

const fibSub = fibonacciObservable.subscribe({
  next(x) {
    console.log(`fib: ${x}`);
  },
  error(err) {
    console.error("Rip", err);
  },
  complete() {
    console.log("Fib over and out");
  },
});

setTimeout(() => {
    fibSub.unsubscribe();
}, 5000);
