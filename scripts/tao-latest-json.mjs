// Ghép `latest.json` cho updater Tauri từ các bộ cài đã ký của mọi đích Windows (ADR-0003: x64 + ARM64).
// Dùng: node scripts/tao-latest-json.mjs <thư mục phát hành> <thẻ vX.Y.Z> <owner/repo> <tệp ghi chú>
// Thư mục phải có đủ cặp `ubnd-app_<ver>_windows-<arch>-setup.exe` + `.sig` cho MỌI đích dưới - thiếu một đích là
// dừng, vì máy kiến trúc đó sẽ không bao giờ thấy bản mới mà không ai hay.
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const [thuMuc, the, repo, tepGhiChu] = process.argv.slice(2);
if (!thuMuc || !/^v\d+\.\d+\.\d+$/.test(the ?? '') || !/^[\w.-]+\/[\w.-]+$/.test(repo ?? '')) {
  console.error('Dùng: node scripts/tao-latest-json.mjs <thư mục> <vX.Y.Z> <owner/repo> [tệp ghi chú]');
  process.exit(2);
}
const ver = the.slice(1);
const DICH = { 'windows-x86_64': 'x64', 'windows-aarch64': 'arm64' };

const platforms = {};
for (const [khoa, arch] of Object.entries(DICH)) {
  const ten = `ubnd-app_${ver}_windows-${arch}-setup.exe`;
  const sig = join(thuMuc, `${ten}.sig`);
  if (!existsSync(join(thuMuc, ten)) || !existsSync(sig)) {
    console.error(`LỖI: thiếu ${ten} hoặc chữ ký .sig của nó`);
    process.exit(1);
  }
  platforms[khoa] = {
    signature: readFileSync(sig, 'utf8').trim(),
    url: `https://github.com/${repo}/releases/download/${the}/${ten}`,
  };
}

const notes = tepGhiChu && existsSync(tepGhiChu) ? readFileSync(tepGhiChu, 'utf8').trim() : `Phiên bản ${ver}`;
const latest = { version: ver, notes, pub_date: new Date().toISOString().replace(/\.\d{3}Z$/, 'Z'), platforms };
writeFileSync(join(thuMuc, 'latest.json'), `${JSON.stringify(latest, null, 2)}\n`);
console.log(`latest.json: ${Object.keys(platforms).join(', ')}`);
