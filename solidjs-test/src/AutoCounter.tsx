import { createSignal, onCleanup } from "solid-js";

const AutoCounter = (props) => {
    const [count, setCount] = createSignal(props.start || 0);
    const timer = setInterval(() => setCount(count() + 1), 1000);
    
    onCleanup(() => clearInterval(timer));

    return <div>{count()}</div>;
};

export default AutoCounter;
