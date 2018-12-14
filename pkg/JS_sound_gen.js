/* tslint:disable */
import * as wasm from './JS_sound_gen_bg';

let cachegetFloat32Memory = null;
function getFloat32Memory() {
    if (cachegetFloat32Memory === null || cachegetFloat32Memory.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory;
}

function getArrayF32FromWasm(ptr, len) {
    return getFloat32Memory().subarray(ptr / 4, ptr / 4 + len);
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}
/**
* @param {number} arg0
* @returns {Float32Array}
*/
export function sound(arg0) {
    const retptr = globalArgumentPtr();
    wasm.sound(retptr, arg0);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getArrayF32FromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 4);
    return realRet;

}

function freeSoundGen(ptr) {

    wasm.__wbg_soundgen_free(ptr);
}
/**
*/
export class SoundGen {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeSoundGen(ptr);
    }

    /**
    * @param {number} arg0
    * @returns {}
    */
    constructor(arg0) {
        this.ptr = wasm.soundgen_new(arg0);
    }
    /**
    * @param {number} arg0
    * @returns {Float32Array}
    */
    sound(arg0) {
        const retptr = globalArgumentPtr();
        wasm.soundgen_sound(retptr, this.ptr, arg0);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getArrayF32FromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 4);
        return realRet;

    }
}

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

