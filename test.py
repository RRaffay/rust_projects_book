import threading
import time


def fibonacci(n):
    if n in (0, 1):
        return n
    else:
        return fibonacci(n-1) + fibonacci(n-2)


def thread_function(n):
    print(f"fib({n}) = {fibonacci(n)}")


if __name__ == "__main__":
    start = time.time()
    threads = []
    for i in range(40, 45):
        x = threading.Thread(target=thread_function, args=(i,))
        threads.append(x)
        x.start()

    for thread in threads:
        thread.join()

    print(f"Total Time: {time.time() - start}")
