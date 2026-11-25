// Ba tệp khai phiên bản phải khớp nhau, và khớp thẻ khi phát hành (`node scripts/kiem-phien-ban.mjs v0.1.0`).
// Updater so ĐÚNG số trong tauri.conf.json: gắn thẻ mà quên tăng số là phát ra bản tự báo "đã mới nhất" mãi mãi.
import { readFileSync } from 'node:fs';

const doc = (p) => readFileSync(new URL(`../${p}`, import.meta.url), 'utf8');
const ban = {
  'package.json': JSON.parse(doc('package.json')).version,
  'src-tauri/tauri.conf.json': JSON.parse(doc('src-tauri/tauri.conf.json')).version,
  'src-tauri/Cargo.toml': /^version\s*=\s*"([^"]+)"/m.exec(doc('src-tauri/Cargo.toml'))?.[1],
};
const the = process.argv[2];
if (the !== undefined) ban[`thẻ ${the}`] = the.replace(/^v/, '');

for (const [noi, so] of Object.entries(ban)) console.log(`${noi.padEnd(28)} ${so}`);
const khac = new Set(Object.values(ban));
if (khac.size !== 1 || !/^\d+\.\d+\.\d+$/.test([...khac][0] ?? '')) {
  console.error('LỖI: phiên bản lệch nhau hoặc không phải dạng X.Y.Z');
  process.exit(1);
}
