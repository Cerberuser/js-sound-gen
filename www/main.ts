// import Worker from 'worker-loader!./gen-worker';

const worker = new Worker('./gen-worker.js');
worker.onerror = (ev: ErrorEvent) => {
    debugger;
};
const context = new AudioContext();

let initialized = false;

worker.onmessage = (ev: MessageEvent) => {

    if (initialized) {
        const sampleRate = context.sampleRate;
        const buffer = context.createBuffer(1, sampleRate * 6, sampleRate);
        const source = context.createBufferSource();

        buffer.copyToChannel(new Float32Array(ev.data.buf as ArrayBuffer), 0, 0);
        console.log(buffer.getChannelData(0));
        source.buffer = buffer;
        source.connect(context.destination);

        source.start();
    } else {
        initialized = true;
        worker.postMessage({sampleRate: context.sampleRate});
    }
};

// import("JS-sound-gen").then(({sound}) => {
//     const sampleRate = context.sampleRate;
//
//     const buf = sound(context.sampleRate).buffer;
//     const buffer = context.createBuffer(1, sampleRate * 6, sampleRate);
//
//     const source = context.createBufferSource();
//
//     buffer.copyToChannel(new Float32Array(buf as ArrayBuffer), 0, 0);
//     console.log(buffer.getChannelData(0));
//     source.buffer = buffer;
//     source.connect(context.destination);
//
//     source.start();
// });


