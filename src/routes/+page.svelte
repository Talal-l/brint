<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";

  async function greet(event: Event) {
    event.preventDefault();
    try {
      await invoke("start_camera");
      goto("/qr");
    } catch (error) {
      console.error(error);
    }
  }
</script>

<main class="container scheme-dark">
  <h1 class="text-3xl font-bold text-center mb-6">
    Self Services Badge Printing
  </h1>
  <button
    onclick={greet}
    class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
    >Scan QR</button
  >
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }
  }
</style>
