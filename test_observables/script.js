import rxjs from "rxjs";

console.log("test observables")

var worker = new Worker('prime.js');
var observable = rxjs.Observable.fromEvent(worker, 'message')
                   .map(function (ev) { return ev.data * 1; })
                   .buffer(Rx.Observable.interval(500))
                   .where(function (x) { return x.length > 0; })
                   .map(function (x) { return x.length; });