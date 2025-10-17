// Run with: node primes_bench.js 2000000

function isPrime(n) {
  if (n < 2) return false;
  if (n % 2 === 0) return n === 2;
  const limit = Math.floor(Math.sqrt(n));
  for (let i = 3; i <= limit; i += 2) {
    if (n % i === 0) return false;
  }
  return true;
}

function main() {
  const n = parseInt(process.argv[2] || "100000000", 10);

  const start = performance.now();
  let count = 0;
  for (let i = 2; i <= n; i++) {
    if (isPrime(i)) count++;
  }
  const elapsed = (performance.now() - start) / 1000;

  console.log(`primes up to ${n}: ${count}`);
  console.log(`elapsed: ${elapsed.toFixed(3)}s`);
}

main();
