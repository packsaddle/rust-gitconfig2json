'use strict';

const fs = require('fs');
const toml = require('toml');
const { URL } = require('url');

const data = toml.parse(fs.readFileSync('./Cargo.toml'));
const repositoryUrl = new URL(data.package.repository);
const [ _, owner, repository ] = repositoryUrl.pathname.split('/');
module.exports = {
  version: data.package.version,
  host: repositoryUrl.origin,
  owner,
  repository,
};
