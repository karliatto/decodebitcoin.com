import { useMemo, useState } from 'react'
import { policy } from "@pkg/decodebitcoin_dot_com.js";

export default function PolicyInfoDecoder()  {
  const [policyStr, setPolicyStr] = useState('and(pk(A),or(pk(B),or(9@pk(C),older(1000))))')
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

  function onDecodePolicy() {
    setError('')
    setResult('')
    try {
      const decoded = policy(policyStr.trim())
      setResult(decoded || '')
    } catch (e: unknown) {
      console.error(e)
      setError(e instanceof Error ? e.message : 'Failed to decode')
    }
  }

  return (
    <div>
      <h1>Miniscript Policy Decoder</h1>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 12, maxWidth: 720 }}>
        <textarea
          value={policyStr}
          onChange={(e) => setPolicyStr(e.target.value)}
          placeholder="and(pk(A),or(pk(B),or(9@pk(C),older(1000))))"
          rows={6}
          style={{ width: '100%', fontFamily: 'monospace', fontSize: 14, padding: 8 }}
        />
        <div>
          <button onClick={onDecodePolicy}>Decode</button>
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


