(function (start, end) {
    var n = start - 1;

    while (n++ < end) {
        var k = Math.sqrt(n);
        var found = false;

        for (var i = 2; !found && i <= k; ++i) {
            found = n % i === 0;
        }

        if (!found) {
            postMessage(n.toString());
        }
    }
})(2, 1e10);
