import { useMemo, useState } from 'react'
import { decode } from "@pkg/decodebitcoin_dot_com.js";

export default function TxDecoder()  {
  const [rawTx, setRawTx] = useState('')
  const [result, setResult] = useState<string>('')
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

  function onDecode() {
    setError('')
    setResult('')
    try {
      const decoded = decode(rawTx.trim())
      setResult(decoded || '')
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : 'Failed to decode')
    }
  }

  return (
    <div>
      <h1>Transaction Decoder</h1>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 12, maxWidth: 720 }}>
        <textarea
          value={rawTx}
          onChange={(e) => setRawTx(e.target.value)}
          placeholder="Paste raw transaction hex"
          rows={6}
          style={{ width: '100%', fontFamily: 'monospace', fontSize: 14, padding: 8 }}
        />
        <div>
          <button onClick={onDecode}>Decode</button>
        </div>
        {error && (
          <div style={{ color: 'tomato' }}>{error}</div>
        )}
        {prettyResult && (
          <pre style={{ background: 'rgba(0,0,0,0.3)', padding: 12, borderRadius: 6, whiteSpace: 'pre-wrap', wordBreak: 'break-word', textAlign: 'left' }}>{prettyResult}</pre>
        )}
      </div>
    </div>
  )
}


