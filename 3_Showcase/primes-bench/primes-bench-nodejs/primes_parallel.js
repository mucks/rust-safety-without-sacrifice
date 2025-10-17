// Run with: node primes_parallel.js 2000000

const { Worker, isMainThread, parentPort, workerData } = require("worker_threads");

function isPrime(n) {
  if (n < 2) return false;
  if (n % 2 === 0) return n === 2;
  const limit = Math.floor(Math.sqrt(n));
  for (let i = 3; i <= limit; i += 2) {
    if (n % i === 0) return false;
  }
  return true;
}

if (!isMainThread) {
  const { start, end } = workerData;
  let count = 0;
  for (let i = start; i <= end; i++) {
    if (isPrime(i)) count++;
  }
  parentPort.postMessage(count);
} else {
  const n = parseInt(process.argv[2] || "100000000", 10);
  const threads = Math.min(8, require("os").cpus().length); // use up to 8 threads
  const chunkSize = Math.ceil(n / threads);
  const startTime = performance.now();

  const promises = [];
  for (let t = 0; t < threads; t++) {
    const start = t * chunkSize + 1;
    const end = Math.min((t + 1) * chunkSize, n);
    promises.push(
      new Promise((resolve) => {
        const worker = new Worker(__filename, { workerData: { start, end } });
        worker.on("message", resolve);
      })
    );
  }

  Promise.all(promises).then((results) => {
    const count = results.reduce((a, b) => a + b, 0);
    const elapsed = (performance.now() - startTime) / 1000;
    console.log(`primes up to ${n}: ${count}`);
    console.log(`elapsed (parallel): ${elapsed.toFixed(3)}s`);
  });
}
