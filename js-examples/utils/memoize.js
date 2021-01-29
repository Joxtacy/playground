export const memoizeSync = (fn) => {
    const cache = {};

    return (...args) => {
        const key = JSON.stringify(args);
        cache[key] = cache[key] || fn(...args);
        return cache[key];
    };
};

export const memoizeAsync = (fn) => {
    const cache = {};

    return async (...args) => {
        const key = JSON.stringify(args);
        cache[key] = cache[key] || fn(...args);
        return cache[key];
    };
};
