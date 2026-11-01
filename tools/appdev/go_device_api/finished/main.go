// Device Inventory API: go run .     then try:  curl http://127.0.0.1:8080/devices
package main

import (
	"context"
	"errors"
	"flag"
	"log/slog"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"
)

func main() {
	// Settings: a flag, or an environment variable (what containers use), or the default.
	addr := flag.String("addr", envOr("ADDR", "127.0.0.1:8080"), "address to listen on")
	data := flag.String("data", envOr("DATA_FILE", "devices.json"), "JSON file to keep the devices in")
	flag.Parse()

	log := slog.New(slog.NewTextHandler(os.Stdout, nil))
	store, err := OpenFileStore(*data)
	if err != nil {
		log.Error("cannot open the store", "err", err)
		os.Exit(1)
	}

	// Timeouts stop slow or stuck clients from holding connections open forever.
	srv := &http.Server{
		Addr:              *addr,
		Handler:           NewAPI(store, log),
		ReadHeaderTimeout: 5 * time.Second,
		ReadTimeout:       10 * time.Second,
		WriteTimeout:      10 * time.Second,
		IdleTimeout:       time.Minute,
	}

	// Ctrl+C (or SIGTERM from Docker, systemd or Kubernetes) cancels ctx.
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

	go func() {
		log.Info("listening", "url", "http://"+*addr, "data", *data)
		if err := srv.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
			log.Error("server failed", "err", err)
			os.Exit(1)
		}
	}()

	<-ctx.Done()
	// Graceful shutdown: stop accepting connections, let requests in progress finish.
	log.Info("shutting down")
	shutdownCtx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	if err := srv.Shutdown(shutdownCtx); err != nil {
		log.Error("shutdown", "err", err)
	}
}

func envOr(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}
