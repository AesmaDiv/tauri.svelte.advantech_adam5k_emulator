export function parseAnalog(bytes: number[]) : number[] {
  let registers = new Uint8Array(bytes);
  let data_view = new DataView(registers.buffer);
  let result = [...Array(64).keys()]
  .map((_,i) => data_view.getUint16(i * 2, false));

  return result;
}

export function parseDigital(bytes: number[]) : number[] {
  return bytes
  .filter((_, i) => i % 2 === 0)
  .reverse()
  .map(v => v.toString(2).padStart(8, '0'))
  .join("")
  .split("")
  .map(v => v === '1' ? 1 : 0)
  .reverse();
}
export function getInheritedProperty(property: string, node: HTMLElement) : string {
  if (!node) return "not found";
  let result = getComputedStyle(node)[property];
  return result || getInheritedProperty(property, node.parentElement);
}
export function getInheritedColor(node: HTMLElement) : string {
  if (!node) return 'rgba(0, 0, 0, 0)';
  let result = getComputedStyle(node).color;
  if (result === 'rgba(0, 0, 0, 0)') result = null;
  return result || getInheritedColor(node.parentElement);
}
export function getInheritedBackgroundColor(node: HTMLElement) : string {
  if (!node) return 'rgba(0, 0, 0, 0)';
  let result = getComputedStyle(node).backgroundColor;
  if (result === 'rgba(0, 0, 0, 0)') result = null;
  return result || getInheritedBackgroundColor(node.parentElement);
}