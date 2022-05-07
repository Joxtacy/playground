<script lang="ts">
  import * as THREE from "three";
  import * as SC from "svelte-cubed";
  import { onMount } from "svelte";

  interface GeneratedBox {
    threejs: THREE.Mesh;
    width: number;
    depth: number;
    direction: "x" | "z";
  }

  const boxHeight = 1;
  const originalBoxSize = 3;

  let gameStarted = false;
  let stack: GeneratedBox[] = [];
  let overhangs: GeneratedBox[] = [];

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
      direction: "x",
    };
  };

  const addLayer = (x: number, z: number, width: number, depth: number, direction: "x" | "z") => {
    const y = boxHeight * stack.length; // Add the new box one layer higher

    const layer = generateBox(x, y, z, width, depth);
    layer.direction = direction;

    stack = [...stack, layer];
  };

  const addOverhang = (x: number, z: number, width: number, depth: number) => {
    const y = boxHeight * (stack.length - 1); // Add the new box on the same layer
    const overhang = generateBox(x, y, z, width, depth);

    overhangs = [...overhangs, overhang];
  };

  const onClick = () => {
    if (!gameStarted) {
      // renderer.setAnimationLoop(animation)
      gameStarted = true;
    } else {
      const topLayer = stack.splice(stack.length - 1)[0];
      const previousLayer = stack[stack.length - 1];

      const direction = topLayer.direction;

      const delta =
        topLayer.threejs.position[direction] - previousLayer.threejs.position[direction];

      const overhangSize = Math.abs(delta);

      const size = direction === "x" ? topLayer.width : topLayer.depth;

      const overlap = size - overhangSize;

      if (overlap > 0) {
        // Cut layer
        const newWidth = direction === "x" ? overlap : topLayer.width;
        const newDepth = direction === "z" ? overlap : topLayer.depth;

        // Update metadata
        topLayer.width = newWidth;
        topLayer.depth = newDepth;

        // Update ThreeJS model
        topLayer.threejs.scale[direction] = overlap / size;
        topLayer.threejs.position[direction] -= delta / 2;

        stack = [...stack, topLayer];

        // Overhang
        const overhangShift = (overlap / 2 + overhangSize / 2) * Math.sign(delta);
        const overhangX =
          direction === "x"
            ? topLayer.threejs.position.x + overhangShift
            : topLayer.threejs.position.x;
        const overhangZ =
          direction === "z"
            ? topLayer.threejs.position.z + overhangShift
            : topLayer.threejs.position.z;
        const overhangWidth = direction === "x" ? overhangSize : newWidth;
        const overhangDepth = direction === "z" ? overhangSize : newDepth;

        addOverhang(overhangX, overhangZ, overhangWidth, overhangDepth);

        // Next layer
        const nextX = direction === "x" ? topLayer.threejs.position.x : -10;
        const nextZ = direction === "z" ? topLayer.threejs.position.z : -10;
        const nextDirection = direction === "x" ? "z" : "x";

        addLayer(nextX, nextZ, newWidth, newDepth, nextDirection);
      }
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
      scale={box.threejs.scale}
    />
  {/each}
  {#each overhangs as box}
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
  <!-- <SC.OrbitControls /> -->
</SC.Canvas>
