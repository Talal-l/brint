<script>
  let { children } = $props();
</script>

<nav>
  <a href="/">Home</a>
  <a href="/qr">QR</a>
  <a href="/badge">Badge</a>
  <a href="/print">Print</a>
</nav>
{@render children()}
