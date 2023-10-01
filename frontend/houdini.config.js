/** @type {import('houdini').ConfigFile} */
const config = {
  watchSchema: {
    url: 'http://localhost:8000/graphql',
    interval: 60 * 1000,
  },
  plugins: {
    "houdini-svelte": {
      static: true,
    }
  },
  scalars: {
    UUID: {
      type: 'string',
    }
  }
}

export default config
