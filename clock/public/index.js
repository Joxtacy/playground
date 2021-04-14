var arc = document.getElementById("arc1");
var date = document.getElementById("date");
var day = document.getElementById("day");
var hour = document.getElementById("hour");
var minute = document.getElementById("minute");
var daysOfTheWeek = [
    "Sun",
    "Mon",
    "Tue",
    "Wed",
    "Thu",
    "Fri",
    "Sat",
];
var interval = 50;
var expected = Date.now() + interval;
var clockStep = function () {
    var now = Date.now();
    var delta = now - expected;
    if (delta > interval) {
        console.warn("[TIMING ERROR] Timeout drifted more than a second.");
    }
    var currentDate = new Date(now);
    var h = currentDate.getHours().toString();
    var m = currentDate.getMinutes().toString();
    hour.textContent = h.length === 1 ? "0" + h : h;
    minute.textContent = m.length === 1 ? "0" + m : m;
    var dayOfMonth = currentDate.getDate();
    var dayOfWeek = currentDate.getDay();
    date.textContent = dayOfMonth.toString();
    day.textContent = daysOfTheWeek[dayOfWeek];
    expected += interval;
    setTimeout(clockStep, Math.max(0, interval - delta));
};
setTimeout(clockStep, interval);
var animateClock = function (timestamp) {
    var current = new Date();
    var angle = (360 / 60000) *
        (current.getMilliseconds() + current.getSeconds() * 1000);
    var dAttribute = describeArc(150, 150, 100, 0, angle);
    arc.setAttribute("d", dAttribute);
    window.requestAnimationFrame(animateClock);
};
window.requestAnimationFrame(animateClock);
var polarToCartesian = function (centerX, centerY, radius, angleInDegrees) {
    var angleInRadians = ((angleInDegrees - 90) * Math.PI) / 180.0;
    return {
        x: centerX + radius * Math.cos(angleInRadians),
        y: centerY + radius * Math.sin(angleInRadians),
    };
};
var describeArc = function (x, y, radius, startAngle, endAngle) {
    var start = polarToCartesian(x, y, radius, endAngle);
    var end = polarToCartesian(x, y, radius, startAngle);
    var largeArcFlag = endAngle - startAngle <= 180 ? "0" : "1";
    var d = [
        "M " + start.x + " " + start.y,
        "A " + radius + " " + radius + " 0 " + largeArcFlag + " 0 " + end.x + " " + end.y,
    ].join(",");
    return d;
};
