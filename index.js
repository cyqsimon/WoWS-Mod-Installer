// Imports
const fs = require("fs-extra");
const semver = require("semver");
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
  const resModsDir = `${gameDir}/bin/${newestVersionDir.name}/res_mods`;
  const semverDirs = fs.readdirSync(resModsDir, {withFileTypes: true}).filter((ent) => ent.isDirectory());
  semverDirs.sort((ent1, ent2) => {
    const semverComp = semver.rcompare(semver.coerce(ent1.name), semver.coerce(ent2.name));
    if (semverComp !== 0) {
      return semverComp;
    } else {
      const prefixRegex = /(\d+\.){2}\d+\.?/;
      let v1Sub = semver.coerce(ent1.name.replace(prefixRegex, ""));
      let v2Sub = semver.coerce(ent2.name.replace(prefixRegex, ""));
      v1Sub = (v1Sub === null) ? semver.coerce("0") : v1Sub;
      v2Sub = (v2Sub === null) ? semver.coerce("0") : v2Sub;
      return semver.rcompare(v1Sub, v2Sub);
    }
  });
  const newestSemverDir = semverDirs[0];
  targetDir = `${gameDir}/bin/${newestVersionDir.name}/res_mods/${newestSemverDir.name}`;
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
