interface Camera {
    id: string;
    name: string;
    url: string;
}

interface PiPState {
    camera: Camera | null;
}

class PiPStore {
    private state = $state<PiPState>({
        camera: null,
    });

    get camera() {
        return this.state.camera;
    }

    get isActive() {
        return this.state.camera !== null;
    }

    open(camera: Camera) {
        this.state.camera = camera;
    }

    close = () => {
        this.state.camera = null;
    };

    toggle(camera: Camera) {
        if (this.state.camera?.id === camera.id) {
            this.close();
        } else {
            this.open(camera);
        }
    }
}

export const pipStore = new PiPStore();
export type { Camera };
