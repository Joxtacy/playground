const arc = document.getElementById("arc1");
const date = document.getElementById("date");
const day = document.getElementById("day");
const hour = document.getElementById("hour");
const minute = document.getElementById("minute");

const daysOfTheWeek = [
    "Sun",
    "Mon",
    "Tue",
    "Wed",
    "Thu",
    "Fri",
    "Sat",
] as const;

const interval = 50;
let expected = Date.now() + interval;

const clockStep = () => {
    const now = Date.now();
    const delta = now - expected;

    if (delta > interval) {
        console.warn("[TIMING ERROR] Timeout drifted more than a second.");
    }

    const currentDate = new Date(now);
    const h = currentDate.getHours().toString();
    const m = currentDate.getMinutes().toString();
    hour.textContent = h.length === 1 ? `0${h}` : h;
    minute.textContent = m.length === 1 ? `0${m}` : m;

    const dayOfMonth = currentDate.getDate();
    const dayOfWeek = currentDate.getDay();
    date.textContent = dayOfMonth.toString();
    day.textContent = daysOfTheWeek[dayOfWeek];

    console.log(interval - delta);
    expected += interval;
    setTimeout(clockStep, Math.max(0, interval - delta));
};

setTimeout(clockStep, interval);

const animateClock = (timestamp: number) => {
    const current = new Date();
    const angle =
        (360 / 60000) *
        (current.getMilliseconds() + current.getSeconds() * 1000);
    const dAttribute = describeArc(150, 150, 100, 0, angle);
    arc.setAttribute("d", dAttribute);

    window.requestAnimationFrame(animateClock);
};
window.requestAnimationFrame(animateClock);

const polarToCartesian = (
    centerX: number,
    centerY: number,
    radius: number,
    angleInDegrees: number
) => {
    const angleInRadians = ((angleInDegrees - 90) * Math.PI) / 180.0;

    return {
        x: centerX + radius * Math.cos(angleInRadians),
        y: centerY + radius * Math.sin(angleInRadians),
    };
};

const describeArc = (
    x: number,
    y: number,
    radius: number,
    startAngle: number,
    endAngle: number
) => {
    const start = polarToCartesian(x, y, radius, endAngle);
    const end = polarToCartesian(x, y, radius, startAngle);

    const largeArcFlag = endAngle - startAngle <= 180 ? "0" : "1";

    const d = [
        `M ${start.x} ${start.y}`,
        `A ${radius} ${radius} 0 ${largeArcFlag} 0 ${end.x} ${end.y}`,
    ].join(",");

    return d;
};
