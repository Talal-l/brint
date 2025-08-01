<script>
  let { children } = $props();
  import "../app.css";
</script>

<nav class="">
  <a href="/">Home</a>
  <a href="/qr">QR</a>
  <a href="/badge">Badge</a>
  <a href="/print">Print</a>
</nav>
{@render children()}
