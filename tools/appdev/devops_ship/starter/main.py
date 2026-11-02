"""Starts the ticket API. Configuration comes from environment variables (twelve-factor)."""
import os
import signal
import sys

from service import make_server


def main():
    host = os.environ.get("HOST", "127.0.0.1")   # the container sets HOST=0.0.0.0
    port = int(os.environ.get("PORT", "8080"))
    server = make_server(host, port)

    # docker stop and Kubernetes send SIGTERM: finish cleanly instead of being killed.
    def stop(signum, frame):
        print('{"msg": "shutting down"}', flush=True)
        sys.exit(0)

    signal.signal(signal.SIGTERM, stop)
    print(f'{{"msg": "listening", "url": "http://{host}:{port}"}}', flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
