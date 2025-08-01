<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { PageProps } from "./$types";
  import { listen } from "@tauri-apps/api/event";
  import { goto } from "$app/navigation";
  let { data }: PageProps = $props();

  let frame = $state("");

  let qrcode = $state("");
  let canvas: HTMLCanvasElement | null = $state(null);


  listen("camera_frame", (event) => {
    frame = event.payload as string;
    const image = new Image();
      image.onload = () => {
        const ctx = canvas?.getContext('2d');
        ctx?.drawImage(image, 0, 0, canvas?.width ?? 0, canvas?.height ?? 0);
      };
      image.src = `data:image/jpeg;base64,${frame}`;


  });

    listen("qrcode_detected", (event) => {
      qrcode = event.payload as string;
      goto("/badge" );
      console.log("qrcode detected");
    });

</script>

<h1>QR Page</h1>

<div style="display: flex; justify-content: center; align-items: center; ">
  <canvas bind:this={canvas} id="qr-canvas" 
  style="border: 1px solid #ccc; width: 100%;"
  ></canvas>
</div>
<div style="display: flex; justify-content: center; align-items: center; ">
  <p>{qrcode}</p>
</div>

