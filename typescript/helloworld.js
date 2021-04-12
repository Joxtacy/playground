var message = "Hello World!";
console.log(message);
var Derp = /** @class */ (function () {
    function Derp() {
    }
    Object.defineProperty(Derp.prototype, "prop", {
        // This prevents treeshaking because transpiling to Object.defineProperty
        // https://mtjody.vercel.app/posts/tree-shaking-ts
        get: function () {
            return 69;
        },
        enumerable: false,
        configurable: true
    });
    return Derp;
}());
;
