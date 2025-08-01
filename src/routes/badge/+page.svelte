<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { PageProps } from "./$types";
  import { listen } from "@tauri-apps/api/event";
  import { goto } from "$app/navigation";

  let { data }: PageProps = $props();

  let frame = $state("");
  let badgeImage = $state("");
  let badgeCanvas: HTMLCanvasElement | null = $state(null);
  let isLiveFeed = $state(true);
  let isPrinting = $state(false);

  listen("camera_frame", (event) => {
    frame = event.payload as string;
    if (isLiveFeed) {
      const image = new Image();
      image.onload = () => {
        drawBadge(image);
      };
      image.src = `data:image/jpeg;base64,${frame}`;
    }
  });

  function drawBadge(image?: HTMLImageElement) {
    if (!badgeCanvas) return;
    
    const ctx = badgeCanvas.getContext('2d');
    if (!ctx) return;

    ctx.fillStyle = '#ffffff';
    ctx.fillRect(0, 0, badgeCanvas.width, badgeCanvas.height);
    
    ctx.strokeStyle = '#000000';
    ctx.lineWidth = 2;
    ctx.strokeRect(10, 10, badgeCanvas.width - 20, badgeCanvas.height - 20);

    const frameWidth = badgeCanvas.width - 40;
    const frameHeight = 200;
    const frameX = 20;
    const frameY = 30;

    ctx.fillStyle = '#f0f0f0';
    ctx.fillRect(frameX, frameY, frameWidth, frameHeight);
    ctx.strokeStyle = '#333333';
    ctx.lineWidth = 3;
    ctx.strokeRect(frameX, frameY, frameWidth, frameHeight);

    if (image) {
      const destX = frameX + 5;
      const destY = frameY + 5;
      const destWidth = frameWidth - 10;
      const destHeight = frameHeight - 10;
      
      ctx.drawImage(image, destX, destY, destWidth, destHeight);
    }

    ctx.fillStyle = '#000000';
    ctx.font = 'bold 24px Arial';
    ctx.textAlign = 'center';
    ctx.fillText(data.name, badgeCanvas.width / 2, frameY + frameHeight + 50);
    
    ctx.font = '18px Arial';
    ctx.fillText(`ID: ${data.id}`, badgeCanvas.width / 2, frameY + frameHeight + 80);
  }

  function capturePhoto() {
    badgeImage = frame;
    isLiveFeed = false;
    
    const image = new Image();
    image.onload = () => {
      drawBadge(image);
    };
    image.src = `data:image/jpeg;base64,${badgeImage}`;

  }

  function retakePhoto() {
    console.log("Retaking photo");
    isLiveFeed = true;
  }

  function printBadge() {
    console.log("Printing badge");
    isPrinting = true;
    capturePhoto();
    
    if (badgeCanvas) {
      const canvasDataUrl = badgeCanvas.toDataURL('image/png');
      const base64Data = canvasDataUrl.split(',')[1];
      invoke("print_badge", { image: base64Data });
    }

  }
  listen("badge_printed", (event) => {
    console.log("Badge printed");
    isPrinting = false;
    // goto("/");

  });
</script>


<h1>Badge Page</h1>

<div style="display: flex; justify-content: center; align-items: center;">
  <canvas bind:this={badgeCanvas} id="badge-canvas" 
  style="border: 1px solid #ccc; width: 300px; height: 400px;"
  width="300" height="400"></canvas>
</div>

<div style="display: flex; justify-content: center; align-items: center; gap: 10px; margin-top: 20px ">
  <button onclick={capturePhoto} disabled={isPrinting}>Capture</button>
  <button onclick={retakePhoto} disabled={isPrinting}>Retake</button> 
  <button onclick={printBadge} disabled={isPrinting}>Print</button> 
</div>