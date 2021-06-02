'use strict'

import fsExtra from 'fs-extra'
import archiver from 'archiver'
import octokitRequest from '@octokit/request'

const { ensureDirSync, createWriteStream, copy, remove, readFile } = fsExtra
const { request } = octokitRequest

const version = process.argv[2]
const accessToken = process.argv[3]
const projectName = 'image-generator'
const bundleOutDirPath = 'libs/'
const archiveExtensionName = 'zip'
const appDirPath = `${bundleOutDirPath}${projectName}-${version}/`

async function bundleCode() {
  ensureDirSync(appDirPath)
  return copy(`${projectName}/Release/`, appDirPath, { recursive: true, filter: (src, dest) => {
    return src.endsWith('/') || src.endsWith('.dll') || src.endsWith('.exe')
  } })
}

async function bundleResources() {
  const resDirPath = `${appDirPath}res/main/`
  ensureDirSync(resDirPath)
  return copy(`${projectName}/res/main/`, resDirPath, { recursive: true })
}

async function bundle() {
  return Promise.all([
    bundleCode(),
    bundleResources()
  ])
}

async function zip() {
  const archive = archiver(archiveExtensionName, { zlib: { level: 9 } })
  archive.on('warning', err => {
    if (err.code == 'ENOENT') console.log(err)
    else throw err
  }).on('error', err => {
    throw err
  }).pipe(createWriteStream(`${bundleOutDirPath}${projectName}-${version}.${archiveExtensionName}`))
  return archive.directory(appDirPath, `${projectName}-${version}/`).finalize()
}

async function publish() {
  const owner = 'ii887522'
  const result = await request('POST /repos/{owner}/{repo}/releases', {
    headers: {
      authorization: `token ${accessToken}`
    },
    owner,
    repo: projectName,
    tag_name: `v${version}`,
    name: `${version}`
  })
  return request('POST /repos/{owner}/{repo}/releases/{release_id}/assets{?name,label}', {
    headers: {
      authorization: `token ${accessToken}`,
      'content-type': 'application/zip'
    },
    baseUrl: 'https://uploads.github.com',
    owner,
    repo: projectName,
    release_id: result.data.id,
    name: `${projectName}-${version}.${archiveExtensionName}`,
    data: await readFile(`${bundleOutDirPath}${[projectName]}-${version}.${archiveExtensionName}`)
  })
}

function clean() {
  remove(`${bundleOutDirPath}${projectName}-${version}`)
  remove(`${bundleOutDirPath}${projectName}-${version}.${archiveExtensionName}`)
}

(async () => {
  await bundle()
  await zip()
  await publish()
  clean()
})()
