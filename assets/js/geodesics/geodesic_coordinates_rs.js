let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
* @param {number} lat1
* @param {number} lon1
* @param {number} lat2
* @param {number} lon2
* @returns {DistBearing}
*/
export function haversine_distance_and_bearing(lat1, lon1, lat2, lon2) {
    const ret = wasm.haversine_distance_and_bearing(lat1, lon1, lat2, lon2);
    return DistBearing.__wrap(ret);
}

/**
* @param {number} lat1
* @param {number} lon1
* @param {number} bearing
* @param {number} distance
* @returns {LocBearing}
*/
export function haversine_location_and_bearing(lat1, lon1, bearing, distance) {
    const ret = wasm.haversine_location_and_bearing(lat1, lon1, bearing, distance);
    return LocBearing.__wrap(ret);
}

/**
* @param {number} lat1
* @param {number} lon1
* @param {number} bearing
* @param {number} distance
* @returns {LocBearing}
*/
export function karney_location_and_bearing(lat1, lon1, bearing, distance) {
    const ret = wasm.karney_location_and_bearing(lat1, lon1, bearing, distance);
    return LocBearing.__wrap(ret);
}

/**
* @param {number} lat1
* @param {number} lon1
* @param {number} lat2
* @param {number} lon2
* @returns {DistBearing}
*/
export function karney_distance_and_bearing(lat1, lon1, lat2, lon2) {
    const ret = wasm.karney_distance_and_bearing(lat1, lon1, lat2, lon2);
    return DistBearing.__wrap(ret);
}

/**
* @param {number} lat1
* @param {number} lon1
* @param {number} bearing
* @param {number} distance
* @returns {LocBearing}
*/
export function vincenty_location_and_bearing(lat1, lon1, bearing, distance) {
    const ret = wasm.vincenty_location_and_bearing(lat1, lon1, bearing, distance);
    return LocBearing.__wrap(ret);
}

/**
* @param {number} lat1
* @param {number} lon1
* @param {number} lat2
* @param {number} lon2
* @returns {DistBearing}
*/
export function vincenty_distance_and_bearing(lat1, lon1, lat2, lon2) {
    const ret = wasm.vincenty_distance_and_bearing(lat1, lon1, lat2, lon2);
    return DistBearing.__wrap(ret);
}

/**
*/
export class DistBearing {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DistBearing.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_distbearing_free(ptr);
    }
    /**
    * @returns {number}
    */
    get distance() {
        const ret = wasm.__wbg_get_distbearing_distance(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set distance(arg0) {
        wasm.__wbg_set_distbearing_distance(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get bearing() {
        const ret = wasm.__wbg_get_distbearing_bearing(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set bearing(arg0) {
        wasm.__wbg_set_distbearing_bearing(this.__wbg_ptr, arg0);
    }
}
/**
*/
export class LocBearing {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LocBearing.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_locbearing_free(ptr);
    }
    /**
    * @returns {number}
    */
    get lat() {
        const ret = wasm.__wbg_get_distbearing_distance(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set lat(arg0) {
        wasm.__wbg_set_distbearing_distance(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get lon() {
        const ret = wasm.__wbg_get_distbearing_bearing(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set lon(arg0) {
        wasm.__wbg_set_distbearing_bearing(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get bearing() {
        const ret = wasm.__wbg_get_locbearing_bearing(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set bearing(arg0) {
        wasm.__wbg_set_locbearing_bearing(this.__wbg_ptr, arg0);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedUint8Memory0 = null;


    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('geodesic_coordinates_rs_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync }
export default __wbg_init;
