<script lang="ts">
  import * as THREE from "three";
  import * as SC from "svelte-cubed";
  import { onMount } from "svelte";

  interface GeneratedBox {
    threejs: THREE.Mesh;
    width: number;
    depth: number;
    direction?: "x" | "z";
  }

  const boxHeight = 1;
  const originalBoxSize = 3;

  const geometry = new THREE.BoxGeometry(originalBoxSize, boxHeight, originalBoxSize);
  const material = new THREE.MeshLambertMaterial({ color: 0xfb8e00 });
  const mesh = new THREE.Mesh(geometry, material);
  mesh.position.set(0, 0, 0);

  let gameStarted = false;
  let stack: GeneratedBox[] = [];

  let cameraYPos = 4;
  let targetY = 0;

  let width: number;
  let height: number;

  onMount(() => {
    width = 10;
    height = width * (window.innerHeight / window.innerWidth);
  });

  const generateBox = (
    x: number,
    y: number,
    z: number,
    width: number,
    depth: number
  ): GeneratedBox => {
    const geometry = new THREE.BoxGeometry(width, boxHeight, depth);

    const color = new THREE.Color(`hsl(${30 + stack.length * 4}, 100%, 50%)`);
    const material = new THREE.MeshLambertMaterial({ color });

    const mesh = new THREE.Mesh(geometry, material);
    mesh.position.set(x, y, z);

    return {
      threejs: mesh,
      width,
      depth,
    };
  };

  const addLayer = (x: number, z: number, width: number, depth: number, direction: "x" | "z") => {
    const y = boxHeight * stack.length;

    const layer = generateBox(x, y, z, width, depth);
    layer.direction = direction;

    stack = [...stack, layer];
  };

  const onClick = () => {
    if (!gameStarted) {
      // renderer.setAnimationLoop(animation)
      gameStarted = true;
    } else {
      const topLayer = stack[stack.length - 1];
      const direction = topLayer.direction;

      // Next layer
      const nextX = direction === "x" ? 0 : -10;
      const nextZ = direction === "z" ? 0 : -10;
      const newWidth = originalBoxSize;
      const newDepth = originalBoxSize;
      const nextDirection = direction === "x" ? "z" : "x";

      addLayer(nextX, nextZ, newWidth, newDepth, nextDirection);
    }
  };

  const animation = () => {
    const speed = 0.15;

    const topLayer = stack.splice(stack.length - 1)[0];
    topLayer.threejs.position[topLayer.direction ?? "z"] += speed;

    stack = [...stack, topLayer];
    if (cameraYPos < boxHeight * (stack.length - 2) + 4) {
      cameraYPos += speed;
      targetY += speed;
    }
  };

  SC.onFrame(() => {
    if (gameStarted) {
      animation();
    }
  });

  addLayer(0, 0, originalBoxSize, originalBoxSize, "z");
  addLayer(-10, 0, originalBoxSize, originalBoxSize, "x");
</script>

<svelte:window on:click={onClick} />

<SC.Canvas antialias>
  {#each stack as box}
    <SC.Mesh
      geometry={box.threejs.geometry}
      material={box.threejs.material}
      position={box.threejs.position}
    />
  {/each}
  <SC.AmbientLight intensity={0.6} />
  <SC.DirectionalLight color={"#ffffff"} intensity={0.6} position={[10, 20, 0]} />
  <SC.OrthographicCamera
    left={width / -2}
    right={width / 2}
    top={height / 2}
    bottom={height / -2}
    near={1}
    far={100}
    position={[4, cameraYPos, 4]}
    target={[0, targetY, 0]}
  />
  <SC.OrbitControls />
</SC.Canvas>
