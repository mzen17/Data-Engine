<script lang="ts">
    import { afterUpdate, onMount } from "svelte";
    export let lines: [number[], number[]][] = [];
    export let save_lines: Function;

    let drawing = false;
    let currentLine: [number[], number[]] | null = null;

    const deleteLineAt = (index: number) => {
      lines = lines.slice(0, index).concat(lines.slice(index + 1));
      save_lines(lines);
      redrawCanvas(); 
    }

    export let imgSrc: string;
    export let next: Function;
  
    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D | null;

    let width = 384;
    let height = 384;
    let scale: number[] = [];

    onMount(async() => {
      canvas = document.getElementById("canvas") as HTMLCanvasElement;
      ctx = canvas.getContext("2d");
      const imgdim = await getImgSize(imgSrc)

      // Scale width to at most 600px
      let x = (imgdim[0] > 500) ? 500 : imgdim[0]
      width = Math.round(x);
      height = Math.round(x * imgdim[1] / imgdim[0])

      var rect = canvas.getBoundingClientRect()
      const scaleX = canvas.width / rect.width;    // relationship bitmap vs. element for x
      const scaleY = canvas.height / rect.height;  // relationship bitmap vs. element for y

      scale = [scaleX, scaleY];
    });

    afterUpdate(() => {
      redrawCanvas();
      var rect = canvas.getBoundingClientRect()
      const scaleX = canvas.width / rect.width;    // relationship bitmap vs. element for x
      const scaleY = canvas.height / rect.height;  // relationship bitmap vs. element for y
      scale = [scaleX, scaleY];
    });

   const distance = (point1: number[], point2: number[]) => {
    const dx = point1[0] - point2[0];
    const dy = point1[1] - point2[1];
    return Math.sqrt(dx * dx + dy * dy);
   }

  const getImgSize = (imageUrl: string) => {
  return new Promise<number[]>((resolve) => {
    const img = new Image();
    img.onload = function () {
      const width = img.naturalWidth;
      const height = img.naturalHeight;
      resolve([width, height]);
    };
    img.src = imageUrl;
  });
};

  
    function handleMouseDown(event: MouseEvent) {
        let pos = getMousePos(canvas, event)

        currentLine = [[pos.x, pos.y], [pos.x, pos.y]];
        drawing = true;
    }
    function handleMouseMove(event: MouseEvent) {
      if (!drawing || !ctx || !currentLine) return;
      let pos = getMousePos(canvas, event)

      currentLine[1] = [pos.x, pos.y];

      if (distance([currentLine[0][0] * scale[0], currentLine[0][1]*scale[1]], [pos.x * scale[0], pos.y * scale[1]]) < 5) {
        currentLine[1] = [pos.x, pos.y];
      } else if (Math.abs(currentLine[0][0] - currentLine[1][0]) < Math.abs(currentLine[0][1] - currentLine[1][1])) {
          currentLine[1] = [currentLine[0][0], pos.y];
          if(isKeydown("Control")) {
            if (currentLine[1][1] > height/2) {
              currentLine[1] = [currentLine[1][0], height];
            }
            else {
              currentLine[1] = [currentLine[1][0], 0];
            }
          }
        }
        else {
          currentLine[1] = [pos.x, currentLine[0][1]];
          if(isKeydown("Control")) {
            if (currentLine[1][0] > width/2) {
              currentLine[1] = [width, currentLine[1][1]];
            }
            else {
              currentLine[1] = [0, currentLine[1][1]];
            }
          }
        }
      
      redrawCanvas();
    }

    function handleMouseUp() {
      if(isKeydown("Shift")) {
        // Expand the current line to be the canvas width or height, whichever the line is facing
        if (currentLine) {
          if (Math.abs(currentLine[0][0] - currentLine[1][0]) < Math.abs(currentLine[0][1] - currentLine[1][1])) {
            currentLine[0] = [currentLine[0][0], 0];
            currentLine[1] = [currentLine[0][0], height];
          }
          else {
            currentLine[0] = [0, currentLine[0][1]];
            currentLine[1] = [width, currentLine[0][1]];
          }
        }
      }
      // Else snap end point to the nearest edge

      if (currentLine) {
        lines = [...lines, currentLine];
      }
      save_lines(lines);
      currentLine = null;
      drawing = false;
    }
  
    function saveLines() {
      next(1);
      lines = [];
      redrawCanvas();
    }
    function saveBackward() {
      next(-1);
      lines = [];
      redrawCanvas();
    }

    function getMousePos(canvas: HTMLCanvasElement, evt:MouseEvent) {
        var rect = canvas.getBoundingClientRect(); // abs. size of element
          return {
            x: (evt.clientX - rect.left),   
            y: (evt.clientY - rect.top) 
          }
        }
  
    function redrawCanvas() {
      if (!ctx) return;
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      lines.forEach((line) => {
        if (!ctx) return;
        ctx.beginPath();
        ctx.moveTo(line[0][0] * scale[0], line[0][1] * scale[1]);
        ctx.lineTo(line[1][0] * scale[0], line[1][1] * scale[1]);
        ctx.stroke();
      });
      if (drawing && currentLine) {
        ctx.beginPath();
        ctx.moveTo(currentLine[0][0] * scale[0], currentLine[0][1] * scale[1]);
        ctx.lineTo(currentLine[1][0] * scale[0], currentLine[1][1] * scale[1]);
        ctx.stroke();
      }
    }

    // Function to check if a specific key is currently being pressed
    function isKeydown(key: any) {
      return !!pressedKeys[key];
    }

    // Object to keep track of pressed keys
    const pressedKeys:any = {};

    // Event listener to track keydown events
    window.addEventListener("keydown", (event) => {
      pressedKeys[event.key] = true;
    });

    // Event listener to track keyup events and remove keys from the pressedKeys object
    window.addEventListener("keyup", (event) => {
      delete pressedKeys[event.key];
    });

  </script>

<div class="flex">
  <div class = "bg-cover mr-2" style="background-image: url({imgSrc}); height: {height}px; width: {width}px;">
    <canvas
        id="canvas"
        on:mousedown={(e) => handleMouseDown(e)}
        on:mousemove={(e) => handleMouseMove(e)}
        on:mouseup={() => handleMouseUp()}
        class="w-full border-2 border-gray-300 mr-2" style="height: {height}px; width: {width}px;"
      >
  </canvas>
  </div>
  <div class="flex-col flex">
    <button id="saveButton" on:click={saveLines} class="bg-green-400 h-8 rounded-md w-32 p-1 mb-2">Next</button>
    <button id="saveButton" on:click={saveBackward} class="bg-green-400 h-8 rounded-md w-32 p-1 mb-2">Backward</button>

    <button id="delete" on:click={() => {lines = [];save_lines(lines);redrawCanvas()}} class="bg-red-700 w-32 h-8 rounded-md p-1">Delete All</button>

    <div class = "h-96 overflow-scroll flex flex-col">
      {#each lines as line, i}
        <button on:click={() => deleteLineAt(i)} class="bg-red-300 my-1 h-8 w-32 rounded-md p-1">Delete Line {i}</button>
      {/each}
    </div>
    </div>
</div>
