// Imports
const fs = require("fs-extra");
const pause = require("node-pause");

// Get conf
const confPath = "./pref.json";
let confStr;
try {
  confStr = fs.readFileSync(confPath);
} catch (e) {
  console.error(e);
  console.error(`Cannot read '${confPath}'. Maybe it does not exist?`);
  promptExit(1);
  return;
}
let conf;
try {
  conf = JSON.parse(confStr);
} catch (e) {
  console.error(e);
  console.error(`'${confPath}' is not a valid JSON.`);
  promptExit(1);
  return;
}
const {gameDir, modsDir} = conf;
if (gameDir === undefined || modsDir === undefined) {
  console.error("'gameDir' and 'modsDir' must be defined in JSON.");
  promptExit(1);
  return;
}

// Find target dir
let targetDir;
try {
  const versionDirs = fs.readdirSync(`${gameDir}/bin`, {withFileTypes: true}).filter((ent) => ent.isDirectory());
  versionDirs.sort((ent1, ent2) => (Number.parseInt(ent2.name, 10) - Number.parseInt(ent1.name, 10)));
  const newestVersionDir = versionDirs[0];
  targetDir = `${gameDir}/bin/${newestVersionDir.name}/res_mods`;
} catch (e) {
  console.error(e);
  console.error(`Failed to locate a target directory in '${gameDir}'. Is it a valid WoWS directory?`);
  promptExit(1);
  return;
}

// Copy files
try {
  fs.copySync(modsDir, targetDir);
  console.log(`Copied all files in '${modsDir}' to '${targetDir}' successfully.`);
  promptExit();
  return;
} catch (e) {
  console.error(e);
  console.error(`Failed to copy contents of '${modsDir}' to '${targetDir}'. Does '${modsDir}' exist?`);
  promptExit(1);
  return;
}

function promptExit(code) {
  pause("Press any key to exit...").then(() => process.exit(code));
}
