import init, { greet, yeet } from "./joxtabot/pkg/joxtabot.js";
await init(Deno.readFile("./joxtabot/pkg/joxtabot_bg.wasm"));

console.log(greet());
console.log(yeet());
