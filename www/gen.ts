import {SoundGen} from "js-sound-gen";

const ctx: Worker = self as any;
let count = 0;
ctx.onmessage = (ev) => {
    if (count < 6) {
        const sound = new SoundGen(ev.data.sampleRate);
        const buf = sound.sound(220 * Math.pow(2, count / 4)).buffer;
        ctx.postMessage({buf}, [buf]);
        count++;
    }
};
