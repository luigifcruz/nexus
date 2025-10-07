import { nexusClient } from "$lib/api/nexus";

type ConnectionStatus = "disconnected" | "connecting" | "connected" | "error";

class ConnectionStore {
    status = $state<ConnectionStatus>("disconnected");
    error = $state<string | null>(null);
    private abortController: AbortController | null = null;

    async connect() {
        if (this.status === "connected" || this.status === "connecting") {
            return;
        }

        this.status = "connecting";
        this.error = null;
        this.abortController = new AbortController();

        try {
            // Test connection by listing replicants (returns Replicant[] now)
            const replicants = await nexusClient.listReplicants();
            this.status = "connected";
        } catch (err) {
            this.status = "error";
            this.error =
                err instanceof Error ? err.message : "Connection failed";
            console.error("Failed to connect to Nexus server:", err);
        }
    }

    disconnect() {
        if (this.abortController) {
            this.abortController.abort();
            this.abortController = null;
        }
        this.status = "disconnected";
        this.error = null;
    }

    async reconnect() {
        this.disconnect();
        await this.connect();
    }
}

export const connectionStore = new ConnectionStore();
