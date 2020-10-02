<script>
    export let size = "45px";
    export let bgColor = "transparent";
    export let color = "#333333";
    export let ref;
</script>

<style>
    .container {
        --line-size: 2px;
        width: var(--size);
        height: var(--size);
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .close {
        position: relative;
        width: 100%;
        height: 100%;
        margin: 0;
        padding: 0;
        top: 0;
        left: 0;

        border: none;
        background: var(--bg-color);
        border-radius: 50%;

        box-shadow: 0px 2px 10px var(--bg-color);
    }

    .close:focus {
        outline: none;
    }

    .close:active {
        position: relative;
        top: 1px;
        left: 1px;
    }

    .close:hover {
        opacity: 1;
    }

    .close::before,
    .close::after {
        position: absolute;
        top: 20%;
        left: calc(50% - (var(--line-size) / 2));
        content: "";
        height: 60%;
        width: var(--line-size);
        background-color: var(--color);
        transition: opacity 0.2s ease;
        opacity: 0.3;
    }
    .close:hover::before,
    .close:hover::after {
        opacity: 1;
    }

    .close::before {
        transform: rotate(45deg);
    }
    .close::after {
        transform: rotate(-45deg);
    }
</style>

<div {ref} style="--size: {size}; --bg-color: {bgColor}; --color: {color};" class="container">
    <button class="close" />
</div>
