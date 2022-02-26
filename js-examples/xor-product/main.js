function xor(a) {
    const rest = a % 8;

    if (rest === 0 || rest === 2 || rest === 4 || rest === 6) {
        return 1;
    } else if (rest === 1 || rest === 5) {
        return 3;
    } else if (rest === 3) {
        return 7;
    } else if (rest === 7) {
        const l = a.toString(2).length;
        return Math.pow(l, 2) - 1;
    }
}

function f(a) {
    const res = [a, 1, a + 1, 0];
    return res[a % 4];
}
function solution(M, N) {
    if (M === N) {
        console.log("result", M);
        return M;
    }

    // let prod = xor(M);
    // for (let i = 1; i < N - M; i++) {
    //     // console.log("prod", prod);
    //     prod = xor(prod);
    // }

    prod = f(N) ^ f(M - 1);

    console.log("result", prod);
    return prod;
}

solution(5, 8);
solution(5, 12);
solution(0, 0);
solution(3, 100000);
