import bool from "./bool";
import float from "./float";
import integer from "./integer";

const choice = (max = 1, min = 0, options = {}) => {
    if (options.float) {
        return float(min, max);
    } else if (options.bool) {
        return bool();
    } else {
        return integer(min, max);
    }
}

export default choice;