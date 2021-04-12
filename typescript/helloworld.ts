let message: string = "Hello World!";
console.log(message);

class Derp {
    // This prevents treeshaking because transpiling to Object.defineProperty
    // https://mtjody.vercel.app/posts/tree-shaking-ts
    get prop() {
        return 69;
    }
};

