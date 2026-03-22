import sharp from 'sharp';
import fs from 'fs';
import pngToIco from 'png-to-ico';

const size = 256;
const blueColor = { r: 59, g: 130, b: 246 }; // Tailwind blue-500

// 创建蓝色背景图片
const image = await sharp({
  create: {
    width: size,
    height: size,
    channels: 3,
    background: blueColor
  }
});

// 保存为 PNG
await image.toFile('src-tauri/icons/icon.png');

// 创建其他尺寸
const sizes = [
  { name: '32x32', width: 32, height: 32 },
  { name: '128x128', width: 128, height: 128 },
  { name: '128x128@2x', width: 256, height: 256 }
];

for (const s of sizes) {
  const resized = await sharp({
    create: {
      width: s.width,
      height: s.height,
      channels: 3,
      background: blueColor
    }
  });
  await resized.toFile(`src-tauri/icons/${s.name}.png`);
}

// 使用 png-to-ico 生成标准格式的 ICO 文件
const pngBuffer = fs.readFileSync('src-tauri/icons/icon.png');
const icoBuffer = await pngToIco([pngBuffer]);
fs.writeFileSync('src-tauri/icons/icon.ico', icoBuffer);

// 复制 PNG 作为 ICNS（占位符）
fs.copyFileSync('src-tauri/icons/icon.png', 'src-tauri/icons/icon.icns');

console.log('Icons created successfully!');
