import './App.css'
import { Routes, Route, Link } from 'react-router-dom'
import About from './pages/About'
import XorBip39 from './pages/XorBip39'
import TxDecoder from './pages/TxDecoder'

export default function App() {
  return (
    <div style={{ minHeight: '100vh' }}>
      <aside style={{ position: 'fixed', left: 0, top: 0, bottom: 0, width: 220, padding: 16, borderRight: '1px solid #333' }}>
        <h2 style={{ marginTop: 0 }}>decodebitcoin.com</h2>
        <nav style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
          <Link to="/" style={{ color: '#fff' }}>Transaction Decoder</Link>
          <Link to="/xorbip39" style={{ color: '#fff' }}>XOR BIP39</Link>
          <Link to="/about" style={{ color: '#fff' }}>About</Link>
        </nav>
      </aside>
      <main style={{ marginLeft: 220, padding: 24 }}>
          <Routes>
          <Route path="/" element={<TxDecoder />} />
          <Route path="/xorbip39" element={<XorBip39 />} />
          <Route path="/about" element={<About />} />
        </Routes>
      </main>
    </div>
  )
}
