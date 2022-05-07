<script lang="ts">
import { onMount } from "svelte";

    import ThreeDee from "./_3d-text.svelte";

    let text = new Date().toLocaleTimeString();
    onMount(() => {
        setInterval(() => {
            text = new Date().toLocaleTimeString();
        }, 1000);
    });
</script>

<ThreeDee {text} />