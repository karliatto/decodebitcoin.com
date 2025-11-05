import { useState } from 'react'
import { xor } from "./../../../pkg";

export default function XorBip39() {
  const [firstWord, setFirstWord] = useState('')
  const [secondWord, setSecondWord] = useState('')
  const [xorResult, setXorResult] = useState('')

  const handleXor = () => {
    try {
      setXorResult(xor(firstWord.trim(), secondWord.trim()))
    } catch (e) {
      setXorResult('')
      console.error(e)
    }
  }

  return (
    <div>
      <h1>XOR BIP39</h1>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 12, maxWidth: 480 }}>
        <input
          type="text"
          value={firstWord}
          onChange={(e) => setFirstWord(e.target.value)}
          placeholder="First word"
          style={{ padding: 8, fontSize: 14 }}
        />
        <input
          type="text"
          value={secondWord}
          onChange={(e) => setSecondWord(e.target.value)}
          placeholder="Second word"
          style={{ padding: 8, fontSize: 14 }}
        />
        <div>
          <button onClick={handleXor}>XOR</button>
        </div>
        {xorResult && (
          <div>
            <strong>Result:</strong>
            <div style={{ marginTop: 8, fontFamily: 'monospace' }}>{xorResult}</div>
          </div>
        )}
      </div>
    </div>
  )
}
