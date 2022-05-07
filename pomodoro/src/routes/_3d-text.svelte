<script lang="ts">
  import * as THREE from "three";
  import * as SC from "svelte-cubed";
  import { FontLoader } from "three/examples/jsm/loaders/FontLoader";
  import { TextGeometry } from "three/examples/jsm/geometries/TextGeometry";
  import type { Font } from "three/examples/jsm/loaders/FontLoader";
  import { onMount } from "svelte";

  let font: Font;
  let background: THREE.Texture;
  export let text: string;

  $: createText = (str: string) => {
    if (!font) return;

    const geometry = new TextGeometry(str, {
      font,
      size: 5,
      height: 1,
      curveSegments: 12,
    });

    geometry.center();

    return geometry;
  };

  onMount(() => {
    new FontLoader().load("/fonts/cascadia_mono_regular.json", (loaded) => {
      font = loaded;
    });

    new THREE.TextureLoader().load("/textures/kiara_1_dawn.jpeg", (loaded) => {
      background = loaded;
      background.mapping = THREE.EquirectangularReflectionMapping;
      background.encoding = THREE.sRGBEncoding;
    });
  });
</script>

<div class="container" class:visible={font && background}>
  <SC.Canvas {background} antialias>
    <SC.Mesh
      geometry={createText(text)}
      material={new THREE.MeshStandardMaterial({
        color: "#ff3e00",
        metalness: 0.3,
        roughness: 0.4,
        opacity: 0.5,
        transparent: false,
      })}
      position={[0, 1, 0]}
    />

    <SC.PerspectiveCamera position={[0, 0, 40]} />
    <SC.OrbitControls
      target={[0, 0, 0]}
      enableZoom={true}
      enablePan={true}
      enableDamping={true}
      maxPolarAngle={Math.PI * 0.5}
    />

    <SC.AmbientLight intensity={0.4} />
    <SC.DirectionalLight position={[2, 1, 5]} intensity={1} color={"#ffffff"} />
  </SC.Canvas>
</div>

<style>
  :global(body) {
    margin: 0;
  }
  .container {
    position: absolute;
    background: rgb(58, 63, 126);
    width: 100%;
    height: 100%;
    top: 0;
  }
  .container :global(canvas) {
    opacity: 0;
    transition: opacity 0.4s;
  }
  .container.visible :global(canvas) {
    opacity: 1;
  }
</style>
