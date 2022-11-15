<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name, somethingElse: "some coding" })
  }

  async function do_something() {
    const msg = await invoke("do_something", { aParam: "henlo" });
    console.log("JOX:", msg);
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}>
      Greet
    </button>
    <button on:click={do_something}>
      Do Something
    </button>
  </div>
  <p>{greetMsg}</p>
</div>
