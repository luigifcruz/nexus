<script lang="ts">
    import { onMount } from "svelte";

    let canvas: HTMLCanvasElement;

    onMount(() => {
        const gl = canvas.getContext('webgl2', { alpha: true, antialias: true })!;
        if (!gl) return;

        let width = window.innerWidth;
        let height = window.innerHeight;
        canvas.width = width;
        canvas.height = height;

        // Vertex shader
        const vertexShader = gl.createShader(gl.VERTEX_SHADER)!;
        gl.shaderSource(vertexShader, `#version 300 es
            in vec2 a_position;
            in float a_size;
            in float a_brightness;
            in float a_twinkleSpeed;
            in float a_twinklePhase;
            in float a_hasBloom;

            uniform vec2 u_resolution;
            uniform float u_time;

            out float v_brightness;
            out float v_hasBloom;

            void main() {
                vec2 clipSpace = (a_position / u_resolution) * 2.0 - 1.0;
                gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
                gl_PointSize = a_size;

                float twinkle = sin(u_time * a_twinkleSpeed + a_twinklePhase);
                float flicker = sin(u_time * a_twinkleSpeed * 3.0 + a_twinklePhase) * 0.15;
                v_brightness = a_brightness * (0.3 + twinkle * 0.5 + flicker + 0.2);
                v_brightness = max(0.1, v_brightness);
                v_hasBloom = a_hasBloom;
            }
        `);
        gl.compileShader(vertexShader);

        // Fragment shader for stars
        const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER)!;
        gl.shaderSource(fragmentShader, `#version 300 es
            precision highp float;

            in float v_brightness;
            in float v_hasBloom;

            out vec4 fragColor;

            void main() {
                vec2 center = gl_PointCoord - 0.5;
                float dist = length(center);

                if (v_hasBloom > 0.5) {
                    // Soft bloom with gradient
                    float alpha = smoothstep(0.5, 0.0, dist) * v_brightness;
                    fragColor = vec4(1.0, 1.0, 1.0, alpha * 0.8);
                } else {
                    // Sharp star
                    float alpha = smoothstep(0.5, 0.3, dist) * v_brightness;
                    fragColor = vec4(1.0, 1.0, 1.0, alpha);
                }
            }
        `);
        gl.compileShader(fragmentShader);

        // Star program
        const starProgram = gl.createProgram()!;
        gl.attachShader(starProgram, vertexShader);
        gl.attachShader(starProgram, fragmentShader);
        gl.linkProgram(starProgram);

        // Burst shaders
        const burstVertexShader = gl.createShader(gl.VERTEX_SHADER)!;
        gl.shaderSource(burstVertexShader, `#version 300 es
            in vec2 a_position;

            uniform vec2 u_resolution;

            out vec2 v_position;

            void main() {
                vec2 clipSpace = (a_position / u_resolution) * 2.0 - 1.0;
                gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
                v_position = a_position;
            }
        `);
        gl.compileShader(burstVertexShader);

        const burstFragmentShader = gl.createShader(gl.FRAGMENT_SHADER)!;
        gl.shaderSource(burstFragmentShader, `#version 300 es
            precision highp float;

            in vec2 v_position;

            uniform vec2 u_center;
            uniform float u_radius;
            uniform float u_maxRadius;

            out vec4 fragColor;

            void main() {
                float dist = length(v_position - u_center);
                float ringWidth = 60.0;
                float innerEdge = u_radius - ringWidth;
                float outerEdge = u_radius;

                // Smooth ring with very soft edges
                float innerFade = smoothstep(innerEdge - 20.0, innerEdge + 15.0, dist);
                float outerFade = smoothstep(outerEdge + 20.0, outerEdge - 15.0, dist);
                float ring = innerFade * outerFade;

                // Progress-based fade
                float progress = u_radius / u_maxRadius;
                float fadeIn = smoothstep(0.0, 0.03, progress);
                float fadeOut = 1.0 - smoothstep(0.2, 0.9, progress);
                float fade = fadeIn * fadeOut;

                // Radial gradient - brighter at leading edge
                float t = clamp((dist - innerEdge) / ringWidth, 0.0, 1.0);
                float gradient = 1.0 - t * t; // quadratic falloff

                // Combine
                float intensity = ring * fade * gradient;
                float alpha = intensity * 0.2;

                // Subtle warm to cool gradient
                vec3 color = mix(vec3(0.5, 0.52, 0.55), vec3(0.9, 0.88, 0.85), gradient);

                fragColor = vec4(color, alpha);
            }
        `);
        gl.compileShader(burstFragmentShader);

        const burstProgram = gl.createProgram()!;
        gl.attachShader(burstProgram, burstVertexShader);
        gl.attachShader(burstProgram, burstFragmentShader);
        gl.linkProgram(burstProgram);

        // Get locations
        const starLocs = {
            position: gl.getAttribLocation(starProgram, 'a_position'),
            size: gl.getAttribLocation(starProgram, 'a_size'),
            brightness: gl.getAttribLocation(starProgram, 'a_brightness'),
            twinkleSpeed: gl.getAttribLocation(starProgram, 'a_twinkleSpeed'),
            twinklePhase: gl.getAttribLocation(starProgram, 'a_twinklePhase'),
            hasBloom: gl.getAttribLocation(starProgram, 'a_hasBloom'),
            resolution: gl.getUniformLocation(starProgram, 'u_resolution'),
            time: gl.getUniformLocation(starProgram, 'u_time')
        };

        const burstLocs = {
            position: gl.getAttribLocation(burstProgram, 'a_position'),
            resolution: gl.getUniformLocation(burstProgram, 'u_resolution'),
            center: gl.getUniformLocation(burstProgram, 'u_center'),
            radius: gl.getUniformLocation(burstProgram, 'u_radius'),
            maxRadius: gl.getUniformLocation(burstProgram, 'u_maxRadius')
        };

        // Create stars
        const starCount = 400;
        const starData = new Float32Array(starCount * 6); // x, y, size, brightness, twinkleSpeed, twinklePhase, hasBloom
        const starDataWithBloom = new Float32Array(starCount * 7);

        for (let i = 0; i < starCount; i++) {
            const idx = i * 7;
            starDataWithBloom[idx] = Math.random() * width;
            starDataWithBloom[idx + 1] = Math.random() * height;
            starDataWithBloom[idx + 2] = 1 + Math.random() * 3; // size
            starDataWithBloom[idx + 3] = 0.2 + Math.random() * 0.5; // brightness
            starDataWithBloom[idx + 4] = 2 + Math.random() * 6; // twinkle speed
            starDataWithBloom[idx + 5] = Math.random() * Math.PI * 2; // phase
            starDataWithBloom[idx + 6] = Math.random() < 0.15 ? 1 : 0; // has bloom
        }

        const starBuffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, starBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, starDataWithBloom, gl.STATIC_DRAW);

        // Full screen quad for bursts
        const quadBuffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, quadBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([
            0, 0, width, 0, 0, height,
            width, 0, width, height, 0, height
        ]), gl.DYNAMIC_DRAW);

        // Bursts
        interface Burst {
            x: number;
            y: number;
            startTime: number;
            speed: number;
            maxRadius: number;
        }
        const bursts: Burst[] = [];

        function createBurst() {
            bursts.push({
                x: Math.random() * width,
                y: Math.random() * height,
                startTime: performance.now(),
                speed: 60 + Math.random() * 40,
                maxRadius: Math.max(width, height) * 1.5
            });
        }

        // Schedule bursts
        let burstTimeout: ReturnType<typeof setTimeout>;
        const scheduleBurst = () => {
            burstTimeout = setTimeout(() => {
                createBurst();
                scheduleBurst();
            }, 15000 + Math.random() * 10000);
        };
        scheduleBurst();

        // Animation
        let animationId: number;

        const render = () => {
            animationId = requestAnimationFrame(render);
            const time = performance.now() / 1000;

            gl.viewport(0, 0, width, height);
            gl.clearColor(0, 0, 0, 1);
            gl.clear(gl.COLOR_BUFFER_BIT);
            gl.enable(gl.BLEND);
            gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

            // Draw bursts first (behind stars)
            gl.useProgram(burstProgram);
            gl.uniform2f(burstLocs.resolution, width, height);

            gl.bindBuffer(gl.ARRAY_BUFFER, quadBuffer);
            gl.enableVertexAttribArray(burstLocs.position);
            gl.vertexAttribPointer(burstLocs.position, 2, gl.FLOAT, false, 0, 0);

            for (let i = bursts.length - 1; i >= 0; i--) {
                const burst = bursts[i];
                const elapsed = (performance.now() - burst.startTime) / 1000;
                const radius = elapsed * burst.speed;

                if (radius > burst.maxRadius) {
                    bursts.splice(i, 1);
                    continue;
                }

                gl.uniform2f(burstLocs.center, burst.x, burst.y);
                gl.uniform1f(burstLocs.radius, radius);
                gl.uniform1f(burstLocs.maxRadius, burst.maxRadius);
                gl.drawArrays(gl.TRIANGLES, 0, 6);
            }

            // Draw stars
            gl.useProgram(starProgram);
            gl.uniform2f(starLocs.resolution, width, height);
            gl.uniform1f(starLocs.time, time);

            gl.bindBuffer(gl.ARRAY_BUFFER, starBuffer);

            const stride = 7 * 4;
            gl.enableVertexAttribArray(starLocs.position);
            gl.vertexAttribPointer(starLocs.position, 2, gl.FLOAT, false, stride, 0);
            gl.enableVertexAttribArray(starLocs.size);
            gl.vertexAttribPointer(starLocs.size, 1, gl.FLOAT, false, stride, 8);
            gl.enableVertexAttribArray(starLocs.brightness);
            gl.vertexAttribPointer(starLocs.brightness, 1, gl.FLOAT, false, stride, 12);
            gl.enableVertexAttribArray(starLocs.twinkleSpeed);
            gl.vertexAttribPointer(starLocs.twinkleSpeed, 1, gl.FLOAT, false, stride, 16);
            gl.enableVertexAttribArray(starLocs.twinklePhase);
            gl.vertexAttribPointer(starLocs.twinklePhase, 1, gl.FLOAT, false, stride, 20);
            gl.enableVertexAttribArray(starLocs.hasBloom);
            gl.vertexAttribPointer(starLocs.hasBloom, 1, gl.FLOAT, false, stride, 24);

            gl.drawArrays(gl.POINTS, 0, starCount);
        };

        render();

        // Resize
        const handleResize = () => {
            width = window.innerWidth;
            height = window.innerHeight;
            canvas.width = width;
            canvas.height = height;

            gl.bindBuffer(gl.ARRAY_BUFFER, quadBuffer);
            gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([
                0, 0, width, 0, 0, height,
                width, 0, width, height, 0, height
            ]), gl.DYNAMIC_DRAW);
        };

        window.addEventListener('resize', handleResize);

        return () => {
            cancelAnimationFrame(animationId);
            clearTimeout(burstTimeout);
            window.removeEventListener('resize', handleResize);
        };
    });
</script>

<canvas bind:this={canvas} class="fixed inset-0 -z-10 pointer-events-none"></canvas>
