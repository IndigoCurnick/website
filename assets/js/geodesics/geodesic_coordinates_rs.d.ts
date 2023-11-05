/* tslint:disable */
/* eslint-disable */
/**
* @param {number} lat1
* @param {number} lon1
* @param {number} lat2
* @param {number} lon2
* @returns {DistBearing}
*/
export function haversine_distance_and_bearing(lat1: number, lon1: number, lat2: number, lon2: number): DistBearing;
/**
* @param {number} lat1
* @param {number} lon1
* @param {number} bearing
* @param {number} distance
* @returns {LocBearing}
*/
export function haversine_location_and_bearing(lat1: number, lon1: number, bearing: number, distance: number): LocBearing;
/**
* @param {number} lat1
* @param {number} lon1
* @param {number} bearing
* @param {number} distance
* @returns {LocBearing}
*/
export function karney_location_and_bearing(lat1: number, lon1: number, bearing: number, distance: number): LocBearing;
/**
* @param {number} lat1
* @param {number} lon1
* @param {number} lat2
* @param {number} lon2
* @returns {DistBearing}
*/
export function karney_distance_and_bearing(lat1: number, lon1: number, lat2: number, lon2: number): DistBearing;
/**
* @param {number} lat1
* @param {number} lon1
* @param {number} bearing
* @param {number} distance
* @returns {LocBearing}
*/
export function vincenty_location_and_bearing(lat1: number, lon1: number, bearing: number, distance: number): LocBearing;
/**
* @param {number} lat1
* @param {number} lon1
* @param {number} lat2
* @param {number} lon2
* @returns {DistBearing}
*/
export function vincenty_distance_and_bearing(lat1: number, lon1: number, lat2: number, lon2: number): DistBearing;
/**
*/
export class DistBearing {
  free(): void;
/**
*/
  bearing: number;
/**
*/
  distance: number;
}
/**
*/
export class LocBearing {
  free(): void;
/**
*/
  bearing: number;
/**
*/
  lat: number;
/**
*/
  lon: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_locbearing_free: (a: number) => void;
  readonly __wbg_get_locbearing_bearing: (a: number) => number;
  readonly __wbg_set_locbearing_bearing: (a: number, b: number) => void;
  readonly __wbg_distbearing_free: (a: number) => void;
  readonly __wbg_get_distbearing_distance: (a: number) => number;
  readonly __wbg_set_distbearing_distance: (a: number, b: number) => void;
  readonly __wbg_get_distbearing_bearing: (a: number) => number;
  readonly __wbg_set_distbearing_bearing: (a: number, b: number) => void;
  readonly __wbg_set_locbearing_lat: (a: number, b: number) => void;
  readonly __wbg_set_locbearing_lon: (a: number, b: number) => void;
  readonly __wbg_get_locbearing_lat: (a: number) => number;
  readonly __wbg_get_locbearing_lon: (a: number) => number;
  readonly haversine_distance_and_bearing: (a: number, b: number, c: number, d: number) => number;
  readonly haversine_location_and_bearing: (a: number, b: number, c: number, d: number) => number;
  readonly karney_location_and_bearing: (a: number, b: number, c: number, d: number) => number;
  readonly karney_distance_and_bearing: (a: number, b: number, c: number, d: number) => number;
  readonly vincenty_location_and_bearing: (a: number, b: number, c: number, d: number) => number;
  readonly vincenty_distance_and_bearing: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
