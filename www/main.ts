import Worker from 'worker-loader!./gen-worker';

const worker = new Worker();
worker.onerror = (ev: ErrorEvent) => {
    debugger;
};
const context = new AudioContext();

worker.onmessage = (ev: MessageEvent) => {
    const sampleRate = context.sampleRate;
    const buffer = context.createBuffer(1, sampleRate * 6, sampleRate);
    const source = context.createBufferSource();

    buffer.copyToChannel(new Float32Array(ev.data.buf as ArrayBuffer), 0, 0);
    console.log(buffer.getChannelData(0));
    source.buffer = buffer;
    source.connect(context.destination);

    source.start();
};
worker.postMessage({sampleRate: context.sampleRate});

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


