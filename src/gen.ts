import("./JS_sound_gen").then(({sound}) => {
    const context = new AudioContext();
    const sampleRate = context.sampleRate;
    const buffer = context.createBuffer(1, sampleRate * 6, sampleRate);
    const source = context.createBufferSource();

    buffer.copyToChannel(sound(sampleRate), 0, 0);
    console.log(buffer.getChannelData(0));
    source.buffer = buffer;
    source.connect(context.destination);

    source.start();
});

