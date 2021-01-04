const { getApiClient } = require('./api')
const { Keyring } = require('@polkadot/api')

async function createEntity (name, countryID, cityID, callback) {
  const api = await getApiClient()
  const keyring = new Keyring({ type: 'sr25519' })
  const BOB = keyring.addFromUri('//Bob', { name: 'Bob default' })

  return api.tx.templateModule
    .createEntity(name, countryID, cityID)
    .signAndSend(BOB, callback)
}

async function getEntity (id) {
  const api = await getApiClient()
  const entity = await api.query.templateModule.entities(id)

  const res = entity.toJSON()
  res.name = hex2a(res.name)
  return res
}

function hex2a (hex) {
  var str = ''
  for (var i = 0; i < hex.length; i += 2) {
    var v = parseInt(hex.substr(i, 2), 16)
    if (v) str += String.fromCharCode(v)
  }
  return str
}

module.exports = {
  createEntity,
  getEntity
}
