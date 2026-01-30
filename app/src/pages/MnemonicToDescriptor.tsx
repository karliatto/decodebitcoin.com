import { useMemo, useState } from 'react'
import { mnemonicToDescriptor, importDescriptor } from "@pkg/decodebitcoin_dot_com.js";

export default function MnemonicToDescriptor() {
  const [mnemonic, setMnemonic] = useState('')
  const [passphrase, setPassphrase] = useState('')
  const [network, setNetwork] = useState('main')
  const [result, setResult] = useState<string>('')
  const [importCommand, setImportCommand] = useState<string>('')
  const [error, setError] = useState<string>('')

  const prettyResult = useMemo(() => {
    if (!result) return ''
    try {
      const parsed = JSON.parse(result)
      return JSON.stringify(parsed, null, 2)
    } catch {
      return result
    }
  }, [result])

  function onDerive() {
    setError('')
    setResult('')
    setImportCommand('')
    try {
      const derived = mnemonicToDescriptor(mnemonic.trim(), passphrase, network)
      setResult(derived || '')

      const parsed = JSON.parse(derived)
      const bip44Desc = importDescriptor(parsed.bip44)
      const bip49Desc = importDescriptor(parsed.bip49)
      const bip84Desc = importDescriptor(parsed.bip84)
      const bip86Desc = importDescriptor(parsed.bip86)
      setImportCommand(`importdescriptors '[${bip44Desc}, ${bip49Desc}, ${bip84Desc}, ${bip86Desc}]'`)
    } catch (e: unknown) {
      console.error(e)
      setError(e instanceof Error ? e.message : 'Failed to derive descriptors')
    }
  }

  return (
    <div>
      <h1>Mnemonic to Descriptor</h1>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 12, maxWidth: 720 }}>
        <textarea
          value={mnemonic}
          onChange={(e) => setMnemonic(e.target.value)}
          placeholder="Enter BIP39 mnemonic (12 or 24 words)"
          rows={3}
          style={{ width: '100%', fontFamily: 'monospace', fontSize: 14, padding: 8 }}
        />
        <input
          type="text"
          value={passphrase}
          onChange={(e) => setPassphrase(e.target.value)}
          placeholder="Passphrase (optional)"
          style={{ padding: 8, fontSize: 14 }}
        />
        <select
          value={network}
          onChange={(e) => setNetwork(e.target.value)}
          style={{ padding: 8, fontSize: 14 }}
        >
          <option value="main">Mainnet</option>
          <option value="test">Testnet</option>
        </select>
        <div>
          <button onClick={onDerive}>Derive Descriptors</button>
        </div>
        {error && (
          <div style={{ color: 'tomato' }}>{error}</div>
        )}
        {prettyResult && (
          <pre style={{ background: 'rgba(0,0,0,0.3)', padding: 12, borderRadius: 6, whiteSpace: 'pre-wrap', wordBreak: 'break-word', textAlign: 'left' }}>{prettyResult}</pre>
        )}
        {importCommand && (
          <>
            <h3>Import into your Bitcoin Core wallet as follows:</h3>
            <pre style={{ background: 'rgba(0,0,0,0.3)', padding: 12, borderRadius: 6, whiteSpace: 'pre-wrap', wordBreak: 'break-word', textAlign: 'left' }}>{importCommand}</pre>
          </>
        )}
      </div>
    </div>
  )
}
