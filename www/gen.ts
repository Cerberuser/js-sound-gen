import {SoundGen} from "js-sound-gen";

const ctx: Worker = self as any;
let count = 0;
const data = {buf: new ArrayBuffer(0)};
ctx.onmessage = (ev) => {
    if (count < 6) {
        const sound = new SoundGen(ev.data.sampleRate);
        setTimeout(() => ctx.postMessage({buf: data.buf}, [data.buf]), 1000);
        data.buf = sound.sound(220 * Math.pow(2, count / 4)).buffer;
        count++;
    }
};
