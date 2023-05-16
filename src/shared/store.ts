import { invoke } from "@tauri-apps/api/tauri";
import { message } from "@tauri-apps/api/dialog";
import { get, writable, type Writable } from "svelte/store";
import type { AdamData } from "./types";
import { parseAnalog, parseDigital } from "./common";

export const ADAMDATA: Writable<AdamData> = writable({
  analog: new Uint16Array(64),
  digital: new Uint8Array(64)
});
export let IPAddress: Writable<string> = writable("127.0.0.1");
export let ServerState: boolean = false;

async function request(bytes: number[]): Promise<number[]> {
  let ip = get(IPAddress);
  return await invoke("send_request", { ip, bytes });
}

export async function switchServerState() : Promise<boolean> {
  return ServerState ? stopServer() : runServer();
}

async function runServer() {
  let ip = get(IPAddress);
  let result = await invoke("run_server", { ip });
  console.log("Запуск сервера по адресу %o -> %o", ip, result);
  ServerState = !!result;

  if (!result) await message("Failed to start server", "Tauri");

  return ServerState;
}
async function stopServer() {
  console.log("Остановка сервера");
  await request([2, 2]);
  await request([2, 2]);
  ServerState = false;

  return ServerState;
}
export async function printMap() {
  const response: number[] = await request([1, 1]);
}

export async function refreshRegisters(): Promise<boolean> {
  let respond_analog: number[] = await request([0,0,0,0,0,6,1,3,0,0,0,64]);
  let respond_digital: number[] = await request([0,0,0,0,0,6,1,1,0,0,0,128]);

  let analog = [];
  let digital = [];

  if (respond_analog.length === 137) analog = parseAnalog(respond_analog.slice(9));
  if (respond_digital.length === 25) digital = parseDigital(respond_digital.slice(9));
 
  if (!analog || !digital) return false;

  ADAMDATA.set({ 
    analog: new Uint16Array(analog),
    digital: new Uint8Array(digital)
  });
  // console.log("Refreshing data", ADAMDATA);

  return true;
}

export async function setAnalog(slot: number, channel: number, value: number) : Promise<boolean> {
  let data_view = new DataView(new ArrayBuffer(2));

  data_view.setUint16(0, slot * 8 + channel, false);
  let address = [...new Uint8Array(data_view.buffer).values()]
  data_view.setUint16(0, value, false);
  let bytes = [...new Uint8Array(data_view.buffer).values()]

  const response: number[] = await request([
    0,0,0,0,0,6,1,6,
    ...address,
    ...bytes
  ]);

  return !!response;
}

export async function setDigital(slot: number, channel: number, value: number) : Promise<boolean> {
  let data_view = new DataView(new ArrayBuffer(2));

  data_view.setUint16(0, slot * 16 + channel, false);
  let address = [...new Uint8Array(data_view.buffer).values()]
  let state = value ? 0xff : 0x00

  const response: number[] = await request([
    0,0,0,0,0,6,1,5,
    ...address,
    state,
    0
  ]);

  return !!response;
}

export async function check() {
  const response: number[] = await request([0,0,0,0,0,6,1,1,0,34,0,2]);
  console.log(response);
}
