<script>
    /**
     * This code is taken from an article listed below.
     * https://css-tricks.com/lazy-loading-images-in-svelte/
     */
    import { onMount } from "svelte";
    import IntersectionObserver from "./IntersectionObserver.svelte";
    import Image from "./Image.svelte";

    export let src;
    export let alt;

    let nativeLoading = false;

    onMount(() => {
        if ("loading" in HTMLImageElement.prototype) {
            nativeLoading = true;
        }
    });
</script>

<IntersectionObserver once={true} let:intersecting>
    {#if intersecting || nativeLoading}
        <Image {alt} {src} />
    {/if}
</IntersectionObserver>
