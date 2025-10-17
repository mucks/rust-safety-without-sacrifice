import time
import math
import argparse

def is_prime(n: int) -> bool:
    if n < 2:
        return False
    if n % 2 == 0:
        return n == 2
    limit = math.isqrt(n)
    i = 3
    while i <= limit:
        if n % i == 0:
            return False
        i += 2
    return True

def main():
    parser = argparse.ArgumentParser(description="Count primes up to N (pure Python)")
    parser.add_argument("n", type=int, nargs="?", default=10_000_000,
                        help="upper bound (default: 10,000,000)")
    args = parser.parse_args()

    start = time.perf_counter()
    count = sum(1 for x in range(2, args.n + 1) if is_prime(x))
    elapsed = time.perf_counter() - start

    print(f"primes up to {args.n}: {count}")
    print(f"elapsed: {elapsed:.3f}s")

if __name__ == "__main__":
    main()
